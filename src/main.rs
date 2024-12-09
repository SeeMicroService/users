use crate::application::{State, UserService};
use crate::domain::types::Result;
use crate::handlers::{
    add_file, add_user, change_password, get_files, info, remove_file, remove_user,
};
use crate::infrastructure::{FileRepository, Postgres, UserRepository};
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod application;
mod domain;
mod handlers;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<()> {
    if dotenvy::dotenv().is_err() {
        println!("Failed to load .env file, trying to use environment variables");
    }
    tracing_subscriber::fmt::init();
    
    let postgres_url = std::env::var("POSTGRES_URL").expect("Expected POSTGRES_URL to be set");
    let hostaddr = std::env::var("HOSTADDR").expect("Expected HOSTADDR to be set");

    let postgres = Arc::new(Postgres::new(&postgres_url).await?);
    let user_repo = Arc::new(UserRepository::new(postgres.clone()));
    let user_service = Arc::new(UserService);
    let file_repo = Arc::new(FileRepository::new(postgres));
    let state = State {
        user_repo,
        user_service,
        file_repo,
    };

    let user = Router::new()
        .route("/info", get(info))
        .route("/add", post(add_user))
        .route("/change_password", patch(change_password))
        .route("/remove", delete(remove_user));

    let files = Router::new()
        .route("/attach_file", post(add_file))
        .route("/detach_file", delete(remove_file))
        .route("/get_filenames", get(get_files));

    let app = Router::new()
        .merge(user)
        .merge(files)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(hostaddr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
