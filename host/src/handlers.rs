use axum::{Router, Json, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use crate::wasm_runner::{Algorithm, AppState, Operation, run_wasm};


#[derive(Deserialize)]
struct Payload { message: String, algorithm: Algorithm }

#[derive(Serialize)]
struct Response { result: String }

async fn encrypt_handler(State(state): State<AppState>, Json(payload): Json<Payload>) -> Json<Response> {
    let result = run_wasm(&state, payload.message, payload.algorithm , Operation::Encrypt).await;
    Json(Response { result })
}

async fn decrypt_handler(State(state): State<AppState>, Json(payload): Json<Payload>) -> Json<Response> {
    let result = run_wasm(&state, payload.message, payload.algorithm, Operation::Decrypt).await;
    Json(Response{ result })
}


pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/encrypt", post(encrypt_handler))
        .route("/decrypt", post(decrypt_handler))
        .with_state(app_state)
}