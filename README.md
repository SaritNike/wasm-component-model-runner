# WASM Component Model Runner

I've built this as a personal learning project to improve my understanding of WebAssembly on the server side. This project demonstrates how to run a native Rust Host (using Wasmtime, Tokio, Axum) that orchestrates multiple Wasm Guests written in different languages (Rust and Python), all communicating via [WASI Preview 2](https://docs.rs/wasip2/latest/wasip2/).

The project focuses on the end-to-end integration while the business logic implemented by the guest and the capbilities provided by the host are rather basic.

The file `wit/cipher.wit` specifies a `encoder-decoder-service` interface which declares two functions `encr` and `decr` (see [Cannot use function name "encrypt" #187](https://github.com/bytecodealliance/componentize-py/issues/187)) and consumes the host's `audit-log` capability. 
## Structure

Currently there is one Rust-based guest providing the Caesar cipher algorithm and a Python-based guest implementing the Vigenère cipher.

- Interface (`wit/cipher.wit`): Defines the interface. Guests must export `encr`and `decr` functions; the Host must provide `audit-log`.
- Host: Wasm-Runner based on [wasmtime](https://github.com/bytecodealliance/wasmtime) that expects guests to implement the `encoder-decoder-service` interface specified in `wit/cipher.wit`. It is compiled to a native Rust binary. Runs the engine and the web server.
- Guest 1: Written in Rust. Compiled to wasm32-wasip2. Implements the [Caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher).
- Guest 2: Written in Python. Compiled to Wasm using [componentize-py](https://github.com/bytecodealliance/componentize-py). Implements the [Vigenère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher).

### Communication:
   - Host calls the guests encrypt/decrypt functions based on the algorithm specified in the incoming requests.
   - Both guests in turn call the audit-log function.

## Prerequisites
- Rust: latest stable version.
- Wasm target: run `rustup target add wasm32-wasip2`
- Python: 3.10+
- componentize-py: For the `vigenere-guest`. Run `pip install requirements.txt`
- Optional: wasm-tools for debugging (`cargo install wasm-tools`) 

## Getting Started

### 1. Configuration
You can configure the algorithms using environment variables:
- `SHIFT_AMOUNT`: Shift offset for the Caesar cipher guest (Default: `3`)
- `VIGENERE_KEYWORD`: Keyword for the Vigenère cipher guest (Default: `WASM`).
- `PORT`: for the Host's axum-based webserver (Default: `3000`)

### 3. Build and Run
Use the Makefile to handle the compilation of both the Rust and Python code.
``` bash
# Build the Host and both Guests
make build

# Directly run the Host (Dev profile with debug info)
make run
```

Once the server is ready and running it should print out on which port it is listening to.

### 4. Usage

Note that the algorithm-name capitlization matters.

**Encrypt using Caesar cipher**  
With default `SHIFT_AMOUNT` of `3`

``` bash
curl -X POST http://localhost:3000/encrypt \
   -H "Content-Type: application/json" \
   -d '{"message": "Hello, World!", "algorithm": "Caesar"}'  
# Output: { "result": "Khoor, Zruog!" }
```

**Decrypt using Caesar cipher**  
With default `SHIFT_AMOUNT` of `3`

``` bash
curl -X POST http://localhost:3000/decrypt \
   -H "Content-Type: application/json" \
   -d '{"message": "Khoor, Zruog!", "algorithm": "Caesar"}'
# Output: { "result": "Hello, World!" }
```

**Encrypt using Vigenère**  
With default `VIGENERE_KEYWORD` set to `WASM`

``` bash
curl -X POST http://localhost:3000/encrypt \
   -H "Content-Type: application/json" \
   -d '{ "message": "Hello, World!", "algorithm": "Vigenere" }'
# Output: { "result": "Dedxk, Wgdhd!" }
```

**Decrypt using Vigenère**  
With default `VIGENERE_KEYWORD` set to `WASM`

``` bash
curl -X POST http://localhost:3000/decrypt \
   -H "Content-Type: application/json" \
   -d '{ "message": "Dedxk, Wgdhd!", "algorithm": "Vigenere" }'
# Output: { "result": "Hello, World!" }
```

# Learning Materials

The list of sources I used to build this:
## Wasmtime and WASIP2
- [Wasmtime Docs](https://docs.wasmtime.dev/introduction.html)
- [Why Component Model?](https://component-model.bytecodealliance.org/design/why-component-model.html)
- [Component Model High-Level Goals](https://github.com/WebAssembly/component-model/blob/main/design/high-level/Goals.md)
- [Component Model High Level Design Choices](https://github.com/WebAssembly/component-model/blob/main/design/high-level/Choices.md)
- [Component Model FAQ](https://github.com/WebAssembly/component-model/blob/main/design/high-level/FAQ.md)
- [Component Model Use Cases](https://github.com/WebAssembly/component-model/blob/main/design/high-level/UseCases.md)
- [WASI Capabilities](https://github.com/WebAssembly/WASI/blob/main/docs/Capabilities.md)
- [Wasm Interface Type (WIT)](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)


### Helpful technical references
- [Axum: Graceful Shutdown Example](https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs#L55)
- [WIT Bindgen Guest Rust](https://github.com/bytecodealliance/wit-bindgen?tab=readme-ov-file#guest-rust)
- [Wasmtime: Component Model Examples](https://docs.wasmtime.dev/api/wasmtime/component/bindgen_examples/_5_all_world_export_kinds/index.html)
- [Wasmtime Component HasData Trait](https://docs.wasmtime.dev/api/wasmtime/component/trait.HasData.html)
