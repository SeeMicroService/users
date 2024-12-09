use crate::domain::types::Error;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::io;

pub(crate) fn handle_error(error: Error, message: Value) -> (StatusCode, Json<Value>) {
    if let Some(err) = error.downcast_ref::<io::Error>() {
        match err.kind() {
            io::ErrorKind::NotFound => return (StatusCode::NOT_FOUND, Json(message)),
            io::ErrorKind::AlreadyExists => return (StatusCode::CONFLICT, Json(message)),
            _ => {}
        }
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": error.to_string() })),
    )
}
