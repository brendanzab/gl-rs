#!/bin/sh

cargo test --verbose --manifest-path gl_generator/Cargo.toml
cargo test --verbose --manifest-path gl/Cargo.toml
cargo test --verbose --manifest-path gl_tests/Cargo.toml
