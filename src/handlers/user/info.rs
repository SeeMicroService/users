use crate::application;
use crate::domain::dto::Credentials;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn info(
    State(state): State<application::State>,
    Json(creds): Json<Credentials>,
) -> (StatusCode, Json<Value>) {
    let result = match creds {
        Credentials {
            id: Some(ref id), ..
        } => state.user_service.info(id, state.user_repo.clone()).await,
        Credentials {
            login: Some(ref login),
            password: Some(ref password),
            ..
        } => {
            state
                .user_service
                .info_by_credentials(login, password, state.user_repo.clone())
                .await
        }
        _ => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"error": "Invalid credentials"})),
            )
        }
    };
    match result {
        Ok(user) => {
            if let Ok(value) = serde_json::to_value(user) {
                (StatusCode::OK, Json(value))
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Unprocessable entity in server's database"})),
                )
            }
        }
        Err(error) => handle_error(error, json!({ "error": "No user found with that id" })),
    }
}
