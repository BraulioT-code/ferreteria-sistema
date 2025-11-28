use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Producto {
    pub id: i32,
    pub codigo_barras: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_venta: Decimal,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductoRequest {
    pub codigo_barras: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_venta: Decimal,
    pub stock: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductoRequest {
    pub codigo_barras: Option<String>,
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub precio_venta: Option<Decimal>,
    pub stock: Option<i32>,
}
