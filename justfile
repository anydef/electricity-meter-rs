# electricity-meter-rs — cross-compile and publish to Gitea
#
# Local dev override: BUILD_TOOLS_DIR=/path/to/build-tools just _bootstrap

set allow-duplicate-variables
set allow-duplicate-recipes

build_tools_dir := ".build/build-tools"
cargo_target    := "arm-unknown-linux-musleabi"
primary_bin     := "electricity_meter_rs"
secondary_bin   := "read-serial"

import? '.build/build-tools/cargo.just'

[private]
default: _bootstrap
    @just --list

[private]
_bootstrap:
    #!/usr/bin/env bash
    set -e
    if [ ! -e {{build_tools_dir}} ]; then
        mkdir -p .build
        if [ -n "${BUILD_TOOLS_DIR:-}" ]; then
            echo "==> Symlinking local build-tools: $BUILD_TOOLS_DIR"
            ln -s "$BUILD_TOOLS_DIR" {{build_tools_dir}}
        else
            echo "==> Cloning build-tools..."
            git clone --depth=1 https://gitea.lab.anydef.de/homelab/build-tools.git {{build_tools_dir}}
        fi
    fi
