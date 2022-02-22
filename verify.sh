#!/bin/sh

export RUSTFLAGS='-D warnings'
export RUSTDOCFLAGS='-D warnings'
set -e

cargo fmt -- --check
cargo clippy --all-features -- -D warnings
cargo doc --all-features --no-deps
cargo test --all-features
cargo test --tests --no-default-features

