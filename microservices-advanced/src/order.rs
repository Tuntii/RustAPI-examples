use reqwest::Client;
use rustapi_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
struct OrderState {
    client: Client,
    registry_url: String,
}

#[derive(Deserialize, Schema)]
struct CreateOrderRequest {
    product_id: String,
    quantity: i32,
}

#[derive(Serialize, Schema)]
struct OrderResponse {
    order_id: String,
    total_price: f64,
    status: String,
}

#[derive(Deserialize, Schema)]
struct Product {
    price: f64,
}

#[derive(Deserialize, Schema)]
struct ServiceInstance {
    url: String,
}

#[derive(Deserialize, Schema)]
struct DiscoverResponse {
    instances: Vec<ServiceInstance>,
}

async fn create_order(
    State(state): State<Arc<OrderState>>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>, ApiError> {
    // 1. Discover Product Service
    let discover_url = format!("{}/discover/product-service", state.registry_url);
    let discover_resp = state
        .client
        .get(&discover_url)
        .send()
        .await
        .map_err(|e| ApiError::internal(format!("Failed to contact registry: {}", e)))?
        .json::<DiscoverResponse>()
        .await
        .map_err(|_| ApiError::internal("Invalid registry response"))?;

    if discover_resp.instances.is_empty() {
        return Err(ApiError::service_unavailable("Product service unavailable"));
    }

    let product_service_url = &discover_resp.instances[0].url;

    // 2. Get Product Details
    let product_url = format!("{}/products/{}", product_service_url, payload.product_id);
    let product = state
        .client
        .get(&product_url)
        .send()
        .await
        .map_err(|e| ApiError::internal(format!("Failed to contact product service: {}", e)))?
        .json::<Product>()
        .await
        .map_err(|_| ApiError::not_found("Product not found or invalid response"))?;

    // 3. Create Order
    let total = product.price * payload.quantity as f64;

    Ok(Json(OrderResponse {
        order_id: uuid::Uuid::new_v4().to_string(),
        total_price: total,
        status: "Created".to_string(),
    }))
}

async fn register_with_registry(registry_url: String, my_port: u16) {
    let client = reqwest::Client::new();
    let my_host = env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
    let my_url = format!("http://{}:{}", my_host, my_port);

    loop {
        let _ = client
            .post(format!("{}/register", registry_url))
            .json(&serde_json::json!({
                "service_name": "order-service",
                "url": my_url
            }))
            .send()
            .await;

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let registry_url = env::var("REGISTRY_URL").unwrap_or("http://localhost:8000".to_string());
    let port = env::var("PORT")
        .unwrap_or("8002".to_string())
        .parse()
        .unwrap();

    let state = Arc::new(OrderState {
        client: Client::new(),
        registry_url: registry_url.clone(),
    });

    tokio::spawn(async move {
        register_with_registry(registry_url, port).await;
    });

    println!("Order Service running on :{}", port);
    RustApi::new()
        .state(state)
        .route("/orders", post(create_order))
        .run(&format!("0.0.0.0:{}", port))
        .await
        .unwrap();
}
