// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{get, App as ActixApp, HttpResponse, HttpServer, Responder};
use tauri::App;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

fn main() {
 tauri::Builder::default()
  .setup(|app| {
      tauri::async_runtime::spawn(
        HttpServer::new(|| {
            ActixApp::new()
               .service(hello)
        })
            .bind(("0.0.0.0:8080"))?
            .run()
      );
      Ok(())
  })
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
