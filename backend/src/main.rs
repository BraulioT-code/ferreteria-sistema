use axum::{
    routing::get,
    Router,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Configurar CORS (necesario para React)
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any); // En desarrollo, permitimos cualquier origen

    // 2. Definir Rutas
    let app = Router::new()
        .route("/api/health", get(|| async { "API de Rust funcionando!" }))
        .layer(cors);

    // 3. Iniciar Servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("-> Servidor Rust escuchando en http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}