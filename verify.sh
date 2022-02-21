#!/bin/sh

export RUSTFLAGS='-D warnings'
export RUSTDOCFLAGS='-D warnings'
set -e

cargo fmt -- --check
cargo clippy --all-features
cargo test --all-features
cargo test --tests
cargo test --tests --no-default-features
cargo doc --all-features --no-deps

