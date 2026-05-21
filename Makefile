# =============================================================================
# electricity-meter-rs — cross-compile and publish to Gitea
# =============================================================================
# Shared build/publish targets live in build-tools/cargo.mk.
#
# Local:  build, package, publish        (uses `cross` for cross-compile)
# CI:     ci-build, ci-package, ci-publish  (called from .gitea/workflows/release.yaml)
#
# Release upload to Gitea is done by akkuman/gitea-release-action in the workflow.
# Deployment to the Pi is owned by snowflakes/ansible/electricity-meter in the
# homelab repo: `make -C ../homelab/snowflakes deploy-electricity-meter`.
# =============================================================================

CARGO_TARGET  := arm-unknown-linux-musleabi
PRIMARY_BIN   := electricity_meter_rs
SECONDARY_BIN := read-serial

BUILD_TOOLS_DIR := .build/build-tools

-include $(BUILD_TOOLS_DIR)/cargo.mk
$(BUILD_TOOLS_DIR)/cargo.mk:
	git clone --depth=1 https://github.com/anydef/build-tools $(BUILD_TOOLS_DIR)
