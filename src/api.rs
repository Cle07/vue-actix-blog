use crate::{AppState, db};
use actix_web::{HttpResponse, Result, get, web};
use chrono::Utc;
use percent_encoding::percent_decode_str;
use serde_json::json;

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
        Some(article) => {
            println!("API: Article found, returning JSON");
            Ok(HttpResponse::Ok().json(article))
        }
        None => {
            println!("API: Article '{}' not found", article_name);
            Ok(HttpResponse::NotFound().json(json!({
                "error": format!("Article '{}' not found", article_name)
            })))
        }
    }
}
