use actix_web::{HttpResponse, Result, get, web};
use chrono::Utc;
use serde_json::json;

use crate::AppState;

// Root API endpoint
#[get("/api")]
pub async fn api_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "This is the API endpoint",
        "version": "1.0",
        "timestamp": Utc::now().to_rfc3339()
    })))
}

// Hello world API endpoint
#[get("/api/actix")]
pub async fn hello(data: web::Data<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({"message": data.app_name.clone()})))
}
