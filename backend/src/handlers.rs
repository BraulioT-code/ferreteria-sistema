use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::{Producto, CreateProductoRequest, UpdateProductoRequest};

// Listar productos
pub async fn get_productos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Producto>>, (StatusCode, String)> {
    let productos = sqlx::query_as::<_, Producto>(r#"SELECT id, codigo_barras, nombre, descripcion, precio_venta, stock, created_at 
       FROM productos 
       ORDER BY id"#)
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(productos))
}

// Obtener un producto por ID
pub async fn get_producto(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Producto>, (StatusCode, String)> {
    let producto = sqlx::query_as::<_, Producto>(r#"SELECT id, codigo_barras, nombre, descripcion, precio_venta, stock, created_at 
       FROM productos WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Producto no encontrado".to_string()))?;

    Ok(Json(producto))
}

// Crear producto
pub async fn create_producto(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductoRequest>,
) -> Result<Json<Producto>, (StatusCode, String)> {
    let producto = sqlx::query_as::<_, Producto>(
        r#"
        INSERT INTO productos (codigo_barras, nombre, descripcion, precio_venta, stock)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(payload.codigo_barras)
    .bind(payload.nombre)
    .bind(payload.descripcion)
    .bind(payload.precio_venta)
    .bind(payload.stock)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        // Manejar error de duplicado (código de barras único)
        if e.to_string().contains("unique constraint") {
            (StatusCode::CONFLICT, "El código de barras ya existe".to_string())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    })?;

    Ok(Json(producto))
}

// Actualizar producto
pub async fn update_producto(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProductoRequest>,
) -> Result<Json<Producto>, (StatusCode, String)> {
    // Primero verificamos si existe
    let _ = sqlx::query("SELECT id FROM productos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Producto no encontrado".to_string()))?;

    // Construcción dinámica de la query (simplificada)
    // Nota: Para producción, un query builder es mejor, pero aquí lo haremos directo o con COALESCE
    // Usaremos COALESCE para actualizar solo lo que viene en el payload, pero eso requiere pasar todos los campos.
    // Otra opción es hacer un fetch primero y luego update, o usar un query dinámico.
    // Usaremos la estrategia de: UPDATE productos SET col = COALESCE($2, col) ...
    
    let producto = sqlx::query_as::<_, Producto>(
        r#"
        UPDATE productos
        SET 
            codigo_barras = COALESCE($2, codigo_barras),
            nombre = COALESCE($3, nombre),
            descripcion = COALESCE($4, descripcion),
            precio_venta = COALESCE($5, precio_venta),
            stock = COALESCE($6, stock)
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(payload.codigo_barras)
    .bind(payload.nombre)
    .bind(payload.descripcion)
    .bind(payload.precio_venta)
    .bind(payload.stock)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(producto))
}

// Eliminar producto
pub async fn delete_producto(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM productos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Producto no encontrado".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
