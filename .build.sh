#!/bin/sh

make gen-lib
rustc --out-dir=$CARGO_OUT_DIR $CARGO_RUSTFLAGS src/gl/lib.rs
