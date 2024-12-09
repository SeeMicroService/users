use crate::application;
use crate::domain::dto::FileDto;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn remove_file(
    State(state): State<application::State>,
    Json(file): Json<FileDto>,
) -> (StatusCode, Json<Value>) {
    match state.file_repo.delete_file(&file.id, &file.filename).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(Value::default())),
        Err(error) => handle_error(error, json!({ "error": "No such file or directory" })),
    }
}
