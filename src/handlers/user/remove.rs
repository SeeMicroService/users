use crate::application;
use crate::domain::dto::UserId;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn remove_user(
    State(state): State<application::State>,
    Json(user_id): Json<UserId>,
) -> (StatusCode, Json<Value>) {
    match state
        .user_service
        .remove(&user_id.id, state.user_repo.clone())
        .await
    {
        Ok(_) => (StatusCode::NO_CONTENT, Json(Value::default())),
        Err(error) => handle_error(error, json!({ "error": "No user found with that id" })),
    }
}
