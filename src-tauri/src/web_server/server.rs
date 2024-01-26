// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{get, App as ActixApp, HttpResponse, HttpServer, Responder};

const IP_ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 8080;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn setup_server() {
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(
                HttpServer::new(|| {
                    ActixApp::new()
                        .service(hello)
                })
                    .bind((IP_ADDRESS, PORT))?
                    .run()
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}