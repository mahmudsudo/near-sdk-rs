#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-target}"
set -e
cd "$(dirname $0)"

pushd ../../cargo-near && cargo install --path . --locked && popd
pushd adder && cargo near metadata && popd
cp $TARGET/near/adder/abi.json ./res/adder-abi.json
cargo build --all --target wasm32-unknown-unknown --release
cp $TARGET/wasm32-unknown-unknown/release/adder.wasm ./res/
cp $TARGET/wasm32-unknown-unknown/release/delegator.wasm ./res/
