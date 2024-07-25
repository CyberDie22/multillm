mod llmconfig;
mod auth;
mod api;

use axum::{
    routing::{get, post},
    handler::Handler,
    http::StatusCode,
    extract::Json, Router,
};
use axum::extract::State;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

mod openai {
    pub mod chatcompletions;
    pub mod error;
}

#[derive(Clone)]
struct AppState {
    config: llmconfig::Config,
}

#[tokio::main]
async fn main() {
    let config_str = std::fs::read_to_string("config.toml").unwrap();
    let config: llmconfig::Config = toml::from_str(&config_str).unwrap();

    println!("models: {:?}", config.models.keys());

    let state = AppState {
        config: config.clone(),
    };

    let app = Router::new()
        .route("/api/v1/ping", get(|| async { (StatusCode::OK, "pong") }))
        .route("/api/v1/chat/completions", post(chat_completions_handler))
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
        )
        .with_state(state);

    let bind_addr = format!("{}:{}", config.global.ip, config.global.port);
    println!("Listening on http://{}", bind_addr);
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn chat_completions_handler(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(req): Json<openai::chatcompletions::request::ChatCompletionRequest>,
) -> (StatusCode, Result<Json<openai::chatcompletions::response::ChatCompletionResponse>, Json<openai::error::Error>>) {
    let api_key = auth::auth_request(&bearer);
    if api_key.is_err() {
        return (StatusCode::UNAUTHORIZED, Err(Json(openai::error::Error {
            error: openai::error::InnerError {
                message: String::from(format!("Incorrect API key provided: {}", bearer.token())),
                _type: String::from("invalid_request_error"),
                param: None,
                code: String::from("invalid_api_key")
            }
        })));
    }



    (StatusCode::OK, Ok(Json(openai::chatcompletions::response::ChatCompletionResponse {
        id: String::from("chatcmpl-0"),
        created: 0,
        model: req.model,
        object: String::from("chat.completion"),
        usage: openai::chatcompletions::response::TokenUsage {
            completion_tokens: 0,
            prompt_tokens: 0,
            total_tokens: 0,
        }
    })))
}