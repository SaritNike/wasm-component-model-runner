use std::{collections::HashMap};

use tokio::signal;
use wasmtime::{Config, Engine, component::{Component, HasSelf, Linker}};
use wasmtime_wasi::{p2::{add_to_linker_async}};

mod wasm_runner;
mod handlers;

use wasm_runner::{EncoderDecoderService, AppState, Guest};

use crate::wasm_runner::Algorithm;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shift_amount = std::env::var("SHIFT_AMOUNT").unwrap_or("3".to_string());
    let vigenere_keyword = std::env::var("VIGENERE_KEYWORD").unwrap_or("WASM".to_string());

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config.wasm_component_model_async(true);

    let engine = Engine::new(&config)?;
    
    let mut linker = Linker::new(&engine);

    // default wasi
    add_to_linker_async(&mut linker)?;

    let mut caesar_guest = Guest{
        component:  Component::from_file(&engine, "./caesar_guest.wasm")?,
        env_vars: vec![],
    };
    caesar_guest.env_vars.push(("SHIFT_AMOUNT".to_string(), shift_amount));

    let mut vigenere_guest = Guest {
        component: Component::from_file(&engine, "./vigenere_guest.wasm")?,
        env_vars: vec![],
    };
    vigenere_guest.env_vars.push(("VIGENERE_KEYWORD".to_string(), vigenere_keyword));

    let mut components = HashMap::new(); 
    components.insert(Algorithm::Caesar, caesar_guest);
    components.insert(Algorithm::Vigenere, vigenere_guest);

    EncoderDecoderService::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
    let app_state = AppState {
        engine, components, linker
    };

    let app = handlers::create_router(app_state);

    println!("Host listening on Port: :{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port)).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal()).await?;

    Ok(())
}

// just copied this from the axum examples
// see https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs#L55
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}