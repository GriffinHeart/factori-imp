.PHONY: test, lint

test:
	cargo test --doc
	cargo nextest run

lint: format clippy

format:
	cargo fmt
	cd factorio-impl; cargo fmt

clippy:
	cargo clippy --all-targets --message-format=short
	cd factorio-impl; cargo clippy --all-targets --message-format=short
