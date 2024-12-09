use crate::application;
use crate::domain::dto::UpdatePassword;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn change_password(
    State(state): State<application::State>,
    Json(new_info): Json<UpdatePassword>,
) -> (StatusCode, Json<Value>) {
    match state
        .user_service
        .change_password(&new_info.id, new_info.password, state.user_repo.clone())
        .await
    {
        Ok(_) => (StatusCode::NO_CONTENT, Json(Value::default())),
        Err(error) => handle_error(error, json!({ "error": "No user found with that id" })),
    }
}
