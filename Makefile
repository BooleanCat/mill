all: check test-unit build test-acceptance

check:
	cargo fmt -- --check
	cargo clippy

build:
	cargo build --release

test-acceptance:
	shellspec

test-unit:
	cargo test