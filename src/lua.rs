use actix_web::{HttpResponse, Result, post, web};
use mlua::prelude::*;
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, timeout};

#[derive(Deserialize)]
pub struct LuaRequest {
    code: String,
}

// Lua code execution endpoint
#[post("/api/lua/run")]
pub async fn lua_run(lua_req: web::Json<LuaRequest>) -> Result<HttpResponse> {
    // Clone the code and move it in a tokio task
    let lua_code = lua_req.code.clone();
    let lua_task = tokio::task::spawn_blocking(move || -> Result<String, String> {
        let lua = Lua::new();
        lua.sandbox(true)
            .expect("If error, check if luau is in features of mlua in cargo.toml");

        // Capture print output
        let output = Arc::new(Mutex::new(Vec::<String>::new()));
        let output_clone = output.clone();

        let print_fn = lua
            .create_function(move |_, args: mlua::MultiValue| {
                let output_str = args
                    .iter()
                    .map(|v| match v {
                        mlua::Value::String(s) => s
                            .to_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|_| "".to_string()),
                        mlua::Value::Integer(i) => i.to_string(),
                        mlua::Value::Number(n) => n.to_string(),
                        mlua::Value::Boolean(b) => b.to_string(),
                        mlua::Value::Nil => "nil".to_string(),
                        _ => format!("{:?}", v), // Fallback for other types
                    })
                    .collect::<Vec<_>>()
                    .join("\t");
                output_clone.lock().unwrap().push(output_str);
                Ok(())
            })
            .map_err(|e| e.to_string())?;
        lua.globals()
            .set("print", print_fn)
            .map_err(|e| e.to_string())?;

        // Execute code
        match lua.load(&lua_code).eval::<mlua::Value>() {
            Ok(_) => {
                let captured_output = output.lock().unwrap().join("\n");
                Ok(captured_output)
            }
            Err(e) => Err(e.to_string()),
        }
    });

    match timeout(Duration::from_secs(5), lua_task).await {
        Ok(Ok(Ok(output))) => Ok(HttpResponse::Ok().json(json!({
            "output": output,
        }))),
        Ok(Ok(Err(lua_error))) => Ok(HttpResponse::BadRequest().json(json!({
            "error": lua_error,
        }))),
        Ok(Err(e)) => {
            log::info!(
                "An internal server error has occured during Lua interpreting:\n{}",
                e
            );
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Task execution failed",
            })))
        }
        Err(_) => Ok(HttpResponse::RequestTimeout().json(json!({
            "error": "Code execution timed out after 5 seconds",
        }))),
    }
}
