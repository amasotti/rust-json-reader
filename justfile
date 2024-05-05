set shell := ["bash", "-c"]

test_json := "test.json"
test_file := ".gitignore"
debug_target := "./target/debug/json_reader"

macos_target := "x86_64-apple-darwin"
linux_target := "x86_64-unknown-linux-gnu"

alias t := test
alias b := build

clean:
    cargo clean

build:
    cargo build

lint: add-clippy
	cargo clippy

fmt: add-fmt
	cargo fmt

test:
	cargo test

add-clippy:
    rustup component add clippy 2> /dev/null

add-fmt:
    rustup component add rustfmt 2> /dev/null

system-info:
  @echo "This is an {{arch()}} machine".

build-for OS:
    @case {{OS}} in \
        "macos") \
            rustup target add {{macos_target}}; \
            cargo build --release --target {{macos_target}}; \
            ;; \
        "linux") \
            rustup target add {{linux_target}}; \
            cargo build --release --target {{linux_target}}; \
            ;; \
        *) \
            echo "Unsupported OS: {{OS}}"; \
            exit 1; \
            ;; \
    esac

run-dev: build
    {{debug_target}} {{test_json}}
    {{debug_target}} {{test_file}}
    {{debug_target}}

default:
    just --list
