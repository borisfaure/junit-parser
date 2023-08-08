#!/usr/bin/env bash
# Script for running check on your rust projects.
set -e
set -x
set -u

run_doc() {
    rustup component add rust-docs
    cargo doc
}

run_fmt() {
    rustup component add rustfmt
    cargo fmt --check
}

run_clippy() {
    rustup component add clippy-preview
    cargo clippy  -- -D warnings
}

run_check() {
    cargo check
}

run_test() {
    cargo test
}

run_build() {
    cargo build
}

run_build_release() {
    cargo build --release
}

case $1 in
    doc)
        run_doc
        ;;
    fmt)
        run_fmt
        ;;
    check)
        run_check
        ;;
    clippy)
        run_clippy
        ;;
    test)
        run_test
        ;;
    build)
        run_build
        ;;
    build-release)
        run_build_release
        ;;
esac
