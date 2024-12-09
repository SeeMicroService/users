use crate::application;
use crate::domain::dto::UserId;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn get_files(
    State(state): State<application::State>,
    Json(id): Json<UserId>,
) -> (StatusCode, Json<Value>) {
    match state.file_repo.get_filenames(&id.id).await {
        Ok(files) => (StatusCode::OK, Json(json!({ "filenames": files }))),
        Err(error) => handle_error(error, json!({ "error": "No user with this id" })),
    }
}
