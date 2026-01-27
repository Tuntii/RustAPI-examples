use dashmap::DashMap;
use rustapi_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct ServiceInstance {
    pub url: String,
    pub last_heartbeat: u64,
}

#[derive(Clone)]
pub struct RegistryState {
    pub services: Arc<DashMap<String, Vec<ServiceInstance>>>,
}

#[derive(Serialize, Deserialize, Schema)]
pub struct RegisterRequest {
    pub service_name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Schema)]
pub struct DiscoverResponse {
    pub instances: Vec<ServiceInstance>,
}

async fn register(
    State(state): State<RegistryState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let instance = ServiceInstance {
        url: payload.url,
        last_heartbeat: now,
    };

    // Remove existing entry for this URL if exists (simple update)
    if let Some(mut instances) = state.services.get_mut(&payload.service_name) {
        instances.retain(|i| i.url != instance.url);
        instances.push(instance);
    } else {
        state.services.insert(payload.service_name, vec![instance]);
    }

    Json("Registered".to_string())
}

async fn discover(
    State(state): State<RegistryState>,
    Path(service_name): Path<String>,
) -> Result<Json<DiscoverResponse>, ApiError> {
    if let Some(instances) = state.services.get(&service_name) {
        Ok(Json(DiscoverResponse {
            instances: instances.clone(),
        }))
    } else {
        Err(ApiError::not_found("Service not found"))
    }
}

#[derive(Serialize)]
pub struct ServicesListResponse {
    pub services: std::collections::HashMap<String, Vec<ServiceInstance>>,
}

async fn list_services(State(state): State<RegistryState>) -> Json<ServicesListResponse> {
    let mut services = std::collections::HashMap::new();
    for entry in &*state.services {
        services.insert(entry.key().clone(), entry.value().clone());
    }
    Json(ServicesListResponse { services })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = RegistryState {
        services: Arc::new(DashMap::new()),
    };

    println!("Service Registry running on :8000");
    RustApi::new()
        .state(state)
        .route("/register", post(register))
        .route("/discover/:service_name", get(discover))
        .route("/services", get(list_services))
        .run("0.0.0.0:8000")
        .await
        .unwrap();
}
