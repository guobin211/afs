#!/usr/bin/env just --justfile

release:
    cargo build --release

lint:
    cargo fmt --all
    cargo clippy --all --fix --allow-dirty --allow-staged

test:
    cargo test

