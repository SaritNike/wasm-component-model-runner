use wasmtime::{ Engine, Store, component::{Component, Linker, bindgen}};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView };


bindgen!({
        path: "../wit",
        world: "encoder-decoder-service",
        imports: { default: async }, // Keep async
        exports: { default: async },
});

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


#[derive(Clone)]
pub struct AppState {
    pub engine: Engine,
    pub component: Component,
    pub linker: Linker<HostState>,
    pub shift_amount: String,
}


pub async fn run_wasm(app: &AppState, input: String, operation: Operation) -> String {
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .env("SHIFT_AMOUNT", &app.shift_amount)
        .build();

    let host_ctx = HostState {
        ctx: wasi_ctx,
        table: ResourceTable::new(),
    };

    let mut store = Store::new(&app.engine, host_ctx);

    let cipher = EncoderDecoderService::instantiate_async(&mut store, &app.component, &app.linker)
        .await.expect("Failed to instantiate");

    match operation {
        Operation::Encrypt => {
            cipher.call_encrypt(&mut store, &input).await.unwrap_or_else(|e| e.to_string())
        }
        Operation::Decrypt => {
            cipher.call_decrypt(&mut store, &input).await.unwrap_or_else(|e| e.to_string())
        }
    }
}