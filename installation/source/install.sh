#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${INSTALL_DIR:-$HOME/.cargo/bin}"
BUILD_PROFILE="${BUILD_PROFILE:-release}"

log() {
    echo "<6>$*" >&2
}

error() {
    echo "<3>Error: $*" >&2
    exit 1
}

check_prerequisites() {
    if ! command -v rustup &> /dev/null; then
        error "rustup not found. Install from https://rustup.rs/"
    fi

    if ! command -v gcc &> /dev/null && ! command -v clang &> /dev/null; then
        error "C compiler (gcc or clang) not found"
    fi

    log "Prerequisites check passed"
}

install_rustowl() {
    log "Installing rustowl to $INSTALL_DIR"

    if [[ "$BUILD_PROFILE" == "release" ]]; then
        cargo install --path . --locked
    else
        cargo install --path . --locked --debug
    fi

    log "Installation complete"
    log "rustowl installed to: $(which rustowl)"
}

main() {
    log "RustOwl source installation script"

    check_prerequisites
    install_rustowl

    log "Run 'rustowl --help' to get started"
}

main "$@"
