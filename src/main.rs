use actix_files as fs;
use actix_web::{App, HttpServer, Result, get, middleware, web};
use std::io;

mod api;
mod db;
mod lua;
mod test;
use api::api_handler;
use lua::lua_run;

// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    app_name: String,
}

// Fallback handler for any route not matched by static files
#[get("{path:.*}")]
async fn spa_fallback() -> Result<fs::NamedFile> {
    match fs::NamedFile::open("./frontend/dist/index.html") {
        Ok(file) => Ok(file.set_content_type(mime::TEXT_HTML)),
        Err(_) => Err(actix_web::error::ErrorNotFound(
            "Vue app not built yet. Run 'cd frontend && bun run build'",
        )),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("🚀 Starting server at http://localhost:8080");
    println!("-----------------------------------------------");
    println!("🔧 To use Vue devtools during development:");
    println!("  1. Keep this server running");
    println!("  2. Open a new terminal and run: cd frontend && bun run dev");
    println!("  3. Access your app at: http://localhost:5173");
    println!("-----------------------------------------------");
    println!("📡 API Endpoints:");
    println!("  - GET /api - Main API info");
    println!("  - GET /api/actix - Hello world endpoint");
    println!("-----------------------------------------------");
    db::init_db().expect("Failed to initialize database");
    println!("📦 Database initialized successfully");
    println!("-----------------------------------------------");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            // Enable CORS for Vue dev server
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add((
                        "Access-Control-Allow-Methods",
                        "GET, POST, PUT, DELETE, OPTIONS",
                    ))
                    .add((
                        "Access-Control-Allow-Headers",
                        "Content-Type, Authorization",
                    )),
            )
            // Register API handlers
            .service(api_handler)
            .service(api::get_article)
            .service(lua_run)
            // Serve static assets from the Vue app's dist directory
            .service(fs::Files::new("/assets", "./frontend/dist/assets"))
            // .service(fs::Files::new("/", "./frontend/dist").index_file("index.html"))
            // Register SPA fallback handler
            .service(spa_fallback)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
