# WASM Component Model Runner

This is a personal learning project to improve my understanding of how WebAssembly on the server side works. It serves as a playground for learning the WebAssembly Component Model. I built this to figure out how to properly integrate a Rust Host (using Wasmtime, Tokio, Axum) with a Wasm Guest (using WASI Preview 2). I wanted to provide an example where the Guest implements a WIT interface and also requests capabilities from the host.

## Structure

Because the goal was to focus on Wasmtime, the business logic implemented by the guest and the capbilities provided to the guest by the host is extremely simple. The file `cipher.wit` specifies a `encoder-decoder-service` world which declares two methods `encrypt` and `decrypt` and imports the host capability `audit-log`.

- Host: Native Rust binary. Runs the engine and the web server.
- Guest: Compiled to wasm32-wasip2. Reads env vars and runs the cipher logic.
- Communication:
    - Host calls the guests encrypt/decrypt functions based on incoming requests.
    - Guest triggers the audit-log function, which is the capability provided to it by the host.

### Main vs Advanced Branch
The code is split up into 2 branches:

- The `main` branch just contains a single guest, which is hardwired inside the host and provides a just a specific environment variable.

- The `advanced` branch is able to dynamically load various guests. The host is therefore acting more like a `function-as-a-service` runner.

## Getting Started

Just use the commands in the Makefile and call `make build` to build both the guest and the host.
Once the build is complete, run the hosts executable.

Alternatively, call `make run` to run the code directly without building the host.

The basic version in the main-branch just runs a caesar-cipher as a basic encryption service. You can configure the shift by setting the environment variable `SHIFT_AMOUNT`. The shift defaults to `3`.

You can encrypt your message with a default shift of `3`.

``` bash
curl -X POST http://localhost:3000/encrypt \
   -H "Content-Type: application/json" \
   -d '{"message": "hello world"}' # { "result": "khoor zruog" }
```

or decrypt it again with

``` bash
curl -X POST http://localhost:3000/decrypt \
   -H "Content-Type: application/json" \
   -d '{"message": "khoor zruog"}' # { "result": "hello world" }
```

# TODO
 - [ ] clean-up the advanced branch for publishing
 - [ ] add integration tests

# Learning Materials

The list of sources I used to build this:
- https://component-model.bytecodealliance.org/design/why-component-model.html
- https://github.com/bytecodealliance/wit-bindgen?tab=readme-ov-file#guest-rust
- https://docs.wasmtime.dev/api/wasmtime/component/bindgen_examples/_5_all_world_export_kinds/index.html
- https://docs.wasmtime.dev/api/wasmtime/component/trait.HasData.html
- https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md
- https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
