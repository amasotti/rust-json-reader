SHELL := /bin/bash

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	@cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

test:
	@cargo test

build-darwin:
	@rustup target add x86_64-apple-darwin
	@cargo build --release --target x86_64-apple-darwin

build-linux:
	@rustup target add x86_64-unknown-linux-gnu
	@cargo build --release --target x86_64-unknown-linux-gnu
