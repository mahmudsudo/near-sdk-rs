#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-target}"
set -e
cd "$(dirname $0)"

pushd ../../cargo-near && cargo install --path . --locked && popd
cargo build --all --target wasm32-unknown-unknown --release
pushd adder && cargo near metadata && popd
cp $TARGET/wasm32-unknown-unknown/release/adder.wasm ./res/
cp $TARGET/wasm32-unknown-unknown/release/delegator.wasm ./res/
cp $TARGET/near/adder/metadata.json ./res/adder-metadata.json
