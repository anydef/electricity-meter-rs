# =============================================================================
# electricity-meter-rs — cross-compile, deploy, and publish to Gitea
# =============================================================================
# Aligned with the homelab Makefile pattern: auto-cloned build-tools,
# .env.tpl loaded via `op inject` (locally) or `load-secrets-action` (CI).
# =============================================================================

SHELL := /bin/bash

BUILD_CONTEXT   := $(CURDIR)
BUILD_TOOLS_DIR := .build/build-tools
ENV_FILE        := $(BUILD_CONTEXT)/.env.tpl

CARGO_TARGET    := arm-unknown-linux-musleabi
RELEASE_DIR     := target/$(CARGO_TARGET)/release
DIST_DIR        := dist
PRIMARY_BIN     := electricity_meter_rs
SECONDARY_BIN   := read-serial
PKG_VERSION     := $(shell awk -F\" '/^version/ {print $$2; exit}' Cargo.toml)
ARTIFACT_NAME   := $(PRIMARY_BIN)-$(PKG_VERSION)-$(CARGO_TARGET).tar.gz

TARGET_MACHINE  := homelab@192.168.3.50
TARGET_PATH     := /home/homelab/electricity-meter-rs/

GITEA_BASE_URL  := https://gitea.lab.anydef.de
GITEA_REPO      := anydef/electricity-meter-rs
RELEASE_TAG     := v$(PKG_VERSION)

# Auto-clone build-tools for load-env-tpl.sh.
$(BUILD_TOOLS_DIR)/load-env-tpl.sh:
	git clone --depth=1 https://github.com/anydef/build-tools $(BUILD_TOOLS_DIR)

# LOAD_ENV: shell snippet that resolves op:// refs from .env.tpl into the
# current shell. Skipped when _OP_LOADED=1 (CI loads via load-secrets-action).
# Used as a prefix in recipe lines: `@$(LOAD_ENV) <next command>`.
LOAD_ENV = if [ -z "$$_OP_LOADED" ] && [ -f "$(ENV_FILE)" ]; then \
		source $(BUILD_TOOLS_DIR)/load-env-tpl.sh $(ENV_FILE); \
	fi;

.PHONY: help build package publish release release-upload deploy run-remote \
        run-all deploy-webserver systemd-start systemd-stop update-systemd

help:
	@echo "Targets:"
	@echo "  build            cross-compile release binaries for $(CARGO_TARGET)"
	@echo "  package          tar release binaries into $(DIST_DIR)/"
	@echo "  release          cross-build + upload binary to Gitea release + publish crate (tag $(RELEASE_TAG))"
	@echo "  release-upload   upload $(DIST_DIR)/*.tar.gz to Gitea release $(RELEASE_TAG)"
	@echo "  publish          publish crate to the Gitea cargo registry"
	@echo "  deploy           scp binaries to $(TARGET_MACHINE)"
	@echo "  run-remote       run read-serial on $(TARGET_MACHINE)"
	@echo "  deploy-webserver build, deploy, and restart systemd unit"
	@echo "  update-systemd   push systemd unit file and restart service"

## Cross-compile release binaries
build:
	cross build --target $(CARGO_TARGET) --release

## Tar release binaries for distribution / release upload
package: build
	mkdir -p $(DIST_DIR)
	tar -C $(RELEASE_DIR) -czf $(DIST_DIR)/$(ARTIFACT_NAME) $(PRIMARY_BIN) $(SECONDARY_BIN)
	@echo "Packaged $(DIST_DIR)/$(ARTIFACT_NAME)"

## Publish the crate to the Gitea cargo registry (requires CARGO_REGISTRIES_GITEA_TOKEN)
publish: $(BUILD_TOOLS_DIR)/load-env-tpl.sh
	@$(LOAD_ENV) \
	if [ -z "$$CARGO_REGISTRIES_GITEA_TOKEN" ]; then \
		echo "CARGO_REGISTRIES_GITEA_TOKEN is not set"; exit 1; \
	fi; \
	cargo publish --registry gitea --allow-dirty

TEA_LOGIN := electricity-meter

## Register/refresh tea login for $(GITEA_BASE_URL) using $$GITEA_TOKEN
tea-login: $(BUILD_TOOLS_DIR)/load-env-tpl.sh
	@$(LOAD_ENV) \
	if [ -z "$$GITEA_TOKEN" ]; then echo "GITEA_TOKEN is not set"; exit 1; fi; \
	tea login add --name $(TEA_LOGIN) --url $(GITEA_BASE_URL) --token $$GITEA_TOKEN >/dev/null 2>&1 || \
	  tea login edit $(TEA_LOGIN) --token $$GITEA_TOKEN >/dev/null

## Upload packaged tarballs to a Gitea release (creates the release if missing)
release-upload: package tea-login
	tea releases create --login $(TEA_LOGIN) --repo $(GITEA_REPO) \
		--tag $(RELEASE_TAG) --title $(RELEASE_TAG) \
		$(addprefix --asset ,$(wildcard $(DIST_DIR)/*.tar.gz))

## Cut a release: cross-build, upload binary to Gitea, publish crate
release: release-upload publish

## scp binaries to the Pi Zero
deploy: systemd-stop
	scp $(RELEASE_DIR)/$(PRIMARY_BIN) $(TARGET_MACHINE):$(TARGET_PATH)
	scp $(RELEASE_DIR)/$(SECONDARY_BIN) $(TARGET_MACHINE):$(TARGET_PATH)

run-remote:
	ssh -t $(TARGET_MACHINE) "cd $(TARGET_PATH) && ./read-serial"

run-all: build deploy run-remote

deploy-webserver: build deploy
	ssh -t $(TARGET_MACHINE) "sudo systemctl restart electricity-meter.service"

systemd-stop:
	ssh -t $(TARGET_MACHINE) "sudo systemctl stop electricity-meter.service"

systemd-start:
	ssh -t $(TARGET_MACHINE) "sudo systemctl start electricity-meter.service"

update-systemd: systemd-stop
	scp ./root/etc/systemd/system/electricity-meter.service $(TARGET_MACHINE):$(TARGET_PATH)
	ssh -t $(TARGET_MACHINE) "sudo cp $(TARGET_PATH)/electricity-meter.service /etc/systemd/system/"
	ssh -t $(TARGET_MACHINE) "sudo systemctl start electricity-meter.service"
