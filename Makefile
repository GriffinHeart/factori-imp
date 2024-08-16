.PHONY: test, lint

test:
	cargo nextest run

lint: format clippy

format:
	cargo fmt
	cd factorio-impl; cargo fmt

clippy:
	cargo clippy --all-targets
	cd factorio-impl; cargo clippy --all-targets
