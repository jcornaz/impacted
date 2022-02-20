#!/bin/bash

export RUSTFLAGS='-D warnings'
export RUSTDOCFLAGS='-D warnings'
set -e

cargo fmt -- --check
cargo test --no-default-features
cargo test
cargo test --all-features
cargo clippy --all-features
cargo doc --all-features --no-deps

