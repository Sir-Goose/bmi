#!/usr/bin/env bash
# Cloudflare Pages build script
# Installs the Rust + trunk toolchain and builds the static site.
set -euo pipefail

echo "=== Building BMI Calculator for Cloudflare Pages ==="

# --- Rust toolchain ---------------------------------------------------------
if ! command -v cargo &>/dev/null; then
    echo ">>> Installing Rust via rustup (minimal profile)..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain stable --profile minimal
    . "$HOME/.cargo/env"
fi
export PATH="$HOME/.cargo/bin:$PATH"

# rust-toolchain.toml ensures the wasm target is installed automatically,
# but add it explicitly as a safety net.
rustup target add wasm32-unknown-unknown 2>/dev/null || true

# --- Trunk (use pre-built binary for speed) ---------------------------------
if ! command -v trunk &>/dev/null; then
    ARCH=$(uname -m)
    case "$ARCH" in
        x86_64)  TRUNK_TARGET="x86_64-unknown-linux-gnu" ;;
        aarch64) TRUNK_TARGET="aarch64-unknown-linux-gnu" ;;
        *)       echo "Unsupported arch: $ARCH"; exit 1 ;;
    esac
    TRUNK_VERSION="v0.21.14"
    TRUNK_URL="https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-${TRUNK_TARGET}.tar.gz"
    echo ">>> Installing trunk $TRUNK_VERSION ($TRUNK_TARGET)..."
    mkdir -p "$HOME/.cargo/bin"
    curl -sSL "$TRUNK_URL" | tar xz -C "$HOME/.cargo/bin"
fi

# --- npm deps (Tailwind CSS v4) ---------------------------------------------
echo ">>> Installing npm dependencies..."
npm ci || npm install

# --- Build ------------------------------------------------------------------
echo ">>> Running trunk build --release..."
trunk build --release

echo "=== Build complete! Output is in dist/ ==="
