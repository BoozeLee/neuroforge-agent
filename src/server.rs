use crate::snn::SpikingNetwork;
use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct AppState {
    pub snn: Arc<Mutex<SpikingNetwork>>,
}

pub async fn run_server(port: u16, snn: Arc<Mutex<SpikingNetwork>>) -> anyhow::Result<()> {
    let state = Arc::new(AppState { snn });

    let app = Router::new()
        .route("/health", get(health))
        .route("/status", get(status))
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");
    info!("NeuroForge status server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "neuroforge" }))
}

async fn status(State(state): State<Arc<AppState>>) -> Json<Value> {
    let net = state.snn.lock().await;
    let neurons: Vec<Value> = net
        .neurons
        .iter()
        .map(|(id, neuron)| {
            json!({
                "id": id,
                "label": neuron.label,
                "membrane_potential": neuron.membrane_potential,
                "threshold": neuron.threshold,
                "refractory": neuron.refractory,
                "fired_count": neuron.fired_count,
                "last_signal": neuron.last_signal,
            })
        })
        .collect();

    Json(json!({
        "tick_count": net.tick_count,
        "total_fires": net.total_fires,
        "neurons": neurons,
    }))
}
