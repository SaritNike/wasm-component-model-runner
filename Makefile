.PHONY: build, build-guest

ifeq ($(OS),Windows_NT)
    SHELL := powershell.exe
    .SHELLFLAGS := -NoProfile -Command
endif

build-guest:
	cargo build --manifest-path=caesar-guest/Cargo.toml --target wasm32-wasip2 --release
	cp caesar-guest/target/wasm32-wasip2/release/caesar_guest.wasm ./

build-host:
	cargo build --manifest-path=host/Cargo.toml --release

build: build-guest build-host

run: build-guest
	cargo run --manifest-path=host/Cargo.toml