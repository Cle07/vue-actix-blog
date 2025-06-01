use actix_web::{HttpResponse, Result, get, web};
use chrono::Utc;
use percent_encoding::percent_decode_str;
use serde_json::json;

use crate::db;

// Root API endpoint
#[get("/api")]
pub async fn api_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "This is the API endpoint",
        "version": "1.0",
        "timestamp": Utc::now().to_rfc3339()
    })))
}

#[get("/api/article/{article_name}")]
pub async fn get_article(path: web::Path<String>) -> Result<HttpResponse> {
    // Decode URI of article name
    let encoded_name = path.into_inner();
    let decoded = percent_decode_str(&encoded_name.clone())
        .decode_utf8()
        .unwrap_or_else(|_| encoded_name.into())
        .to_string();
    // Remove extension if present, but don't fail if no extension exists
    let article_name = decoded.split('.').next().unwrap_or(&decoded).to_string();
    // Use the get_from_name method to retrieve the article
    match db::Article::get_from_name(&article_name) {
        Some(article) => Ok(HttpResponse::Ok().json(article)),
        None => Ok(HttpResponse::NotFound().json(json!({
            "error": format!("Article '{}' not found", article_name)
        }))),
    }
}
