current_dir = $(shell pwd)

build-dev:
	cargo build

run-dev: build-dev
	cargo run

build-release:
	docker build --network=host -t "rust-rest" .

.PHONY: build-dev run-dev build-release
