build:
	cargo build

release:
	cargo build --release

test:
	cargo test

run:
	cargo run -- $(pattern) $(filepath)

check:
	cargo check

fmt:
	cargo fmt -- --check

deps:
	cargo machete --fix || true
