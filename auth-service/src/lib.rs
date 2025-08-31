// This struct encapsulates our application-related logic.
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use std::error::Error;

pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;

use app_state::AppState;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/hello", get(hello_handler))
        .route("/login", post(routes::login_handler))
        .route("/verify-2fa", post(routes::verify_2fa_handler))
        .route("/logout", post(routes::logout_handler))
        .route("/verify-token", post(routes::verify_token_handler))
        .route("/signup", post(routes::signup))
        .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application {
            server,
            address,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, Web3 World!</h1>")
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}









