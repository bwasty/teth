#!/bin/bash
# Requires cargo-cov: https://github.com/kennytm/cov
cargo +nightly cov clean
CARGO_INCREMENTAL=0 cargo +nightly cov test
# CARGO_INCREMENTAL=0 cargo +nightly cov run
cargo +nightly cov report --open
