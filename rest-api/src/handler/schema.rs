use axum::response::Json;
use serde_json::Value;

lazy_static::lazy_static! {
    static ref SCHEMA: Value = serde_json::to_value(common::model::RootSchema::build()).unwrap();
}

pub async fn handler() -> Json<&'static Value> {
    Json(&SCHEMA)
}
