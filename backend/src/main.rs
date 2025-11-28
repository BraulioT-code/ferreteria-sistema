use axum::{
    routing::{get, post, patch, delete},
    Router,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
};
use tower_http::cors::{CorsLayer, Any};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

mod models;
mod handlers;

#[tokio::main]
async fn main() {
    // Cargar variables de entorno
    dotenv().ok();

    // 0. Configurar Base de Datos
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada en .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Fallo al conectar a la base de datos");

    println!("-> Conexión a base de datos establecida");

    // 1. Configurar CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any);

    // 2. Definir Rutas
    let app = Router::new()
        .route("/api/health", get(|| async { "API de Rust funcionando!" }))
        .route("/api/productos", get(handlers::get_productos).post(handlers::create_producto))
        .route("/api/productos/:id", get(handlers::get_producto).patch(handlers::update_producto).delete(handlers::delete_producto))
        .with_state(pool)
        .layer(cors);

    // 3. Iniciar Servidor
    // 1. Usa TcpListener de Tokio
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    // 2. Usa axum::serve para servir la aplicación en ese listener
    axum::serve(listener, app)
        .await
        .unwrap();
}