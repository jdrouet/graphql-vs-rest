use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct ServerError {
    status: StatusCode,
    payload: serde_json::Value,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (self.status, Json(self.payload)).into_response()
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(origin: sqlx::Error) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            payload: serde_json::json!({ "reason": origin.to_string() }),
        }
    }
}

impl From<validator::ValidationErrors> for ServerError {
    fn from(origin: validator::ValidationErrors) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            payload: serde_json::to_value(origin).unwrap(),
        }
    }
}
