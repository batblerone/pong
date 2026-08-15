#!/usr/bin/env bash
set -e
# Script to build the web assets for the project
# Assumes that the Rust toolchain and Emscripten are properly set up
# Usage: ./web-build.sh [crate_name] [release|debug]
#
# Configuration needede in .cargo/config.toml
# [target.wasm32-unknown-emscripten]
# rustflags = [
#     "-C", "link-args=-sSIDE_MODULE=2",
#     "-C", "llvm-args=-enable-emscripten-cxx-exceptions=0",
#     "-Z", "default-visibility=hidden",
#     "-Z", "link-native-libraries=no",
# ]
#
# Configuration needed  in Cargo.toml
# [dependencies]
# godot = "0.5.5"

# [lib]
# crate-type = ["cdylib"]

# [features]
# # Ensure that enabling `nothreads` for the main crate also enables
# # that feature for other crates.
# wasm = ["godot/experimental-wasm", "godot/lazy-function-tables"]
# # Markers
# nothreads = []
# threads = []

# wasm-nothreads = ["wasm", "nothreads", "godot/experimental-wasm-nothreads"]

# Navigate to the root directory of the project
cd "$(dirname "$0")"

# Crate name argument
CRATE_NAME=${1:-grust}
# Optional profile argument (default: debug)
PROFILE=${2:-debug}

# Cargo only accepts --release flag; debug is the default and needs no flag
if [ "$PROFILE" = "release" ]; then
    PROFILE_FLAG="--release"
else
    PROFILE_FLAG=""
fi

echo "Building crate: $CRATE_NAME with profile: $PROFILE"

# Make thread build
RUSTFLAGS="-C link-args=-pthread \
-C target-feature=+atomics \
-C link-args=-sSIDE_MODULE=2 \
-C llvm-args=-enable-emscripten-cxx-exceptions=0 \
-Z default-visibility=hidden \
-Z link-native-libraries=no" cargo +nightly build -Zbuild-std --features wasm,threads --target wasm32-unknown-emscripten $PROFILE_FLAG

mv target/wasm32-unknown-emscripten/$PROFILE/$CRATE_NAME.wasm \
   target/wasm32-unknown-emscripten/$PROFILE/$CRATE_NAME.threads.wasm

# Make non-thread build
cargo +nightly build --features wasm-nothreads -Zbuild-std --target wasm32-unknown-emscripten $PROFILE_FLAG
