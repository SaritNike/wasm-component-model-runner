use wasmtime::{Config, Engine, component::{Component, HasSelf, Linker}};
use wasmtime_wasi::{p2::{add_to_linker_async}};

mod wasm_runner;
mod handlers;

use wasm_runner::{EncoderDecoderService, AppState};




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shift_amount = std::env::var("SHIFT_AMOUNT").unwrap_or("3".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config.wasm_component_model_async(true);

    let engine = Engine::new(&config)?;

    let component = Component::from_file(&engine, "./caesar_guest.wasm")?;
    
    let mut linker = Linker::new(&engine);

    // default wasi
    add_to_linker_async(&mut linker)?;

    
    EncoderDecoderService::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
    let app_state = AppState {
        engine, component, linker,
        shift_amount: shift_amount.clone(),
    };

    let app = handlers::create_router(app_state);

    println!("Host started with SHIFT_AMOUNT: {} on Port: :{}", shift_amount, port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

