use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{Pool, Postgres};

use crate::{
    AppState,
    models::{NewProduct, Product}, // Asume que NewProduct y Product están definidos en models.rs
};

/// POST /api/productos
pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<NewProduct>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    // El método .await? permite usar el operador ? para manejar errores de SQLX de forma concisa.
    // Usamos RETURNING * para obtener el producto recién creado, incluyendo el ID y la fecha.
    let res = sqlx::query_as::<_, Product>(
        "INSERT INTO productos (nombre, descripcion, precio, stock, codigo_barras) 
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(payload.nombre)
    .bind(payload.descripcion)
    .bind(payload.precio)
    .bind(payload.stock)
    .bind(payload.codigo_barras)
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(product) => {
            // Retorna el producto creado con el estado 201 Created
            Ok((StatusCode::CREATED, Json(product)))
        }
        Err(e) => {
            // Manejo de errores (ej: restricción UNIQUE violada en codigo_barras)
            eprintln!("Error al insertar producto: {}", e);
            
            // Puedes agregar lógica para distinguir errores específicos de la DB
            let error_response = serde_json::json!({
                "error": "Error al procesar la solicitud",
                "message": format!("Fallo en la base de datos: {}", e),
            });
            
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

// ...

/// GET /api/productos
pub async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    
    let res = sqlx::query_as::<_, Product>(
        "SELECT id, nombre, descripcion, precio, stock, codigo_barras, creado_en 
         FROM productos
         ORDER BY creado_en DESC",
    )
    .fetch_all(&state.db)
    .await;

    match res {
        Ok(products) => {
            // Retorna la lista de productos
            Ok(Json(products))
        }
        Err(e) => {
            eprintln!("Error al listar productos: {}", e);
            // Retorna un error 500
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Error al obtener la lista de productos.".to_string()))
        }
    }
}