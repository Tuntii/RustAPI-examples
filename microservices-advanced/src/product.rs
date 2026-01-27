use rustapi_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize, Schema)]
struct Product {
    id: String,
    name: String,
    price: f64,
}

async fn get_products() -> Json<Vec<Product>> {
    Json(vec![
        Product {
            id: "1".to_string(),
            name: "Laptop".to_string(),
            price: 999.99,
        },
        Product {
            id: "2".to_string(),
            name: "Mouse".to_string(),
            price: 29.99,
        },
    ])
}

async fn get_product(Path(id): Path<String>) -> Result<Json<Product>, ApiError> {
    match id.as_str() {
        "1" => Ok(Json(Product {
            id: "1".to_string(),
            name: "Laptop".to_string(),
            price: 999.99,
        })),
        "2" => Ok(Json(Product {
            id: "2".to_string(),
            name: "Mouse".to_string(),
            price: 29.99,
        })),
        _ => Err(ApiError::not_found("Product not found")),
    }
}

async fn register_with_registry(registry_url: String, my_port: u16) {
    let client = reqwest::Client::new();
    let my_host = env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
    // In docker-compose, HOSTNAME is set. locally, we might need a better way, but for now:
    let my_url = format!("http://{}:{}", my_host, my_port);

    loop {
        let resp = client
            .post(format!("{}/register", registry_url))
            .json(&serde_json::json!({
                "service_name": "product-service",
                "url": my_url
            }))
            .send()
            .await;

        match resp {
            Ok(_) => println!("Registered with registry"),
            Err(e) => println!("Failed to register: {}", e),
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT")
        .unwrap_or("8001".to_string())
        .parse()
        .unwrap();
    let registry_url = env::var("REGISTRY_URL").unwrap_or("http://localhost:8000".to_string());

    tokio::spawn(async move {
        register_with_registry(registry_url, port).await;
    });

    println!("Product Service running on :{}", port);
    RustApi::new()
        .route("/products", get(get_products))
        .route("/products/:id", get(get_product))
        .run(&format!("0.0.0.0:{}", port))
        .await
        .unwrap();
}
