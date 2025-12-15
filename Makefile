.PHONY: build, build-guests, build-host

ifeq ($(OS),Windows_NT)
    SHELL := powershell.exe
    .SHELLFLAGS := -NoProfile -Command
endif

build-guests:
	componentize-py -d wit/cipher.wit -w encoder-decoder-service componentize -p vigenere-guest app -o vigenere_guest.wasm
	cargo build --manifest-path=caesar-guest/Cargo.toml --target wasm32-wasip2 --release
	cp caesar-guest/target/wasm32-wasip2/release/caesar_guest.wasm ./

build-host:
	cargo build --manifest-path=host/Cargo.toml --release

build: build-guests build-host

run: build-guests
	cargo run --manifest-path=host/Cargo.toml