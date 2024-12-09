use crate::application;
use crate::domain::dto::UserData;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn add_user(
    State(state): State<application::State>,
    Json(credentials): Json<UserData>,
) -> (StatusCode, Json<Value>) {
    match state.user_service.add(credentials, state.user_repo.clone()).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "user_id": id }))),
        Err(error) => handle_error(error, json!({ "error": "User already exists" })),
    }
}
