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

# `cross` for local dev; CI overrides to plain `cargo` since the job runtime
# (the rust-cross-arm-musl image) already provides the cross toolchain.
CARGO          ?= cross

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
CI_RELEASE_TAG  := main-$(shell git rev-parse --short HEAD 2>/dev/null)

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
        run-all deploy-webserver systemd-start systemd-stop update-systemd \
        ci-build ci-package ci-release-upload ci-publish ci-release tea-login

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
	$(CARGO) build --target $(CARGO_TARGET) --release

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

# -----------------------------------------------------------------------------
# CI targets — invoked from .gitea/workflows/release.yaml.
# The job runs inside the rust-cross-arm-musl image, so we bypass `cross`
# and tag the Gitea release by short commit SHA so each main push is unique.
# -----------------------------------------------------------------------------

ci-build:
	$(MAKE) CARGO=cargo build

ci-package:
	$(MAKE) CARGO=cargo package

## Upload artifacts to a Gitea release via the API (avoids `tea` and its xdg-open quirks).
ci-release-upload: ci-package
	@if [ -z "$$GITEA_TOKEN" ]; then echo "GITEA_TOKEN is not set"; exit 1; fi
	@tag="$(CI_RELEASE_TAG)"; \
	echo "Creating Gitea release $$tag for $(GITEA_REPO)..."; \
	resp=$$(curl -fsS -X POST \
		-H "Authorization: token $$GITEA_TOKEN" \
		-H "Content-Type: application/json" \
		-d "$$(jq -nc --arg tag $$tag '{tag_name:$$tag, name:$$tag}')" \
		"$(GITEA_BASE_URL)/api/v1/repos/$(GITEA_REPO)/releases"); \
	release_id=$$(echo "$$resp" | jq -r '.id // empty'); \
	if [ -z "$$release_id" ]; then echo "Failed to create release: $$resp"; exit 1; fi; \
	echo "Created release id=$$release_id"; \
	for f in $(DIST_DIR)/*.tar.gz; do \
		echo "Uploading $$f..."; \
		curl -fsS -X POST \
			-H "Authorization: token $$GITEA_TOKEN" \
			-F "attachment=@$$f" \
			"$(GITEA_BASE_URL)/api/v1/repos/$(GITEA_REPO)/releases/$$release_id/assets" >/dev/null; \
	done; \
	echo "Done."

## Publish crate, tolerating "already exists" so version bumps are the trigger.
ci-publish: $(BUILD_TOOLS_DIR)/load-env-tpl.sh
	@$(LOAD_ENV) \
	if [ -z "$$CARGO_REGISTRIES_GITEA_TOKEN" ]; then \
		echo "CARGO_REGISTRIES_GITEA_TOKEN is not set"; exit 1; \
	fi; \
	out=$$(cargo publish --registry gitea --allow-dirty 2>&1); rc=$$?; \
	echo "$$out"; \
	if [ $$rc -ne 0 ] && ! echo "$$out" | grep -qiE "already (exists|uploaded)"; then \
		exit $$rc; \
	fi

ci-release: ci-release-upload ci-publish

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
