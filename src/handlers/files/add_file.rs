use crate::application;
use crate::domain::dto::FileDto;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn add_file(
    State(state): State<application::State>,
    Json(file): Json<FileDto>,
) -> (StatusCode, Json<Value>) {
    match state.file_repo.add_file(&file.id, &file.filename).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "user_id": id }))),
        Err(error) => handle_error(error, json!({ "error": "File already exists" })),
    }
}
