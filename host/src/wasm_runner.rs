use std::collections::HashMap;
use serde::Deserialize;
use wasmtime::{ Engine, Store, component::{Component, Linker, bindgen}};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView };


bindgen!({
        path: "../wit",
        world: "encoder-decoder-service",
        imports: { default: async }, // Keep async
        exports: { default: async },
});

// the audit-log capability provided by the host to the guests
impl saritnike::cipher::audit_log::Host for HostState {
    async fn auditrecord(&mut self,action: String, detail: String) {
        println!("[AUDIT] Action: {} | Detail: {}", action, detail);
    }
}

pub struct HostState {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.ctx, table: &mut self.table }
    }
}


// start of the runner logic
pub enum Operation {
    Encrypt,
    Decrypt,
}

#[derive(Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum Algorithm {
    Caesar,
    Vigenere
}


#[derive(Clone)]
pub struct AppState {
    pub engine: Engine,
    pub components: HashMap<Algorithm,Guest>,
    pub linker: Linker<HostState>,
}

#[derive(Clone)]
pub struct Guest {
    pub component: Component,
    pub env_vars: Vec<(String, String)>
}


pub async fn run_wasm(app: &AppState, input: String, algorithm: Algorithm, operation: Operation) -> String {
    
    let guest : &Guest;

    match algorithm {
        Algorithm::Caesar => guest = &app.components[&Algorithm::Caesar],
        Algorithm::Vigenere => guest = &app.components[&Algorithm::Vigenere],
    }


    let wasi_ctx =  WasiCtxBuilder::new()
        .inherit_stdio()
        .envs(&guest.env_vars)
        .build();

    let host_ctx = HostState {
        ctx: wasi_ctx,
        table: ResourceTable::new(),
    };

    let mut store = Store::new(&app.engine, host_ctx);

    let cipher = EncoderDecoderService::instantiate_async(&mut store, &guest.component, &app.linker)
        .await.expect("Failed to instantiate");

    match operation {
        Operation::Encrypt => {
            cipher.call_encr(&mut store, &input).await.unwrap_or_else(|e| e.to_string())
        }
        Operation::Decrypt => {
            cipher.call_decr(&mut store, &input).await.unwrap_or_else(|e| e.to_string())
        }
    }
}