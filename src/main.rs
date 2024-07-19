use axum::{http::StatusCode, response::IntoResponse, routing, Json, Router};
use serde::Serialize;
use tracing::instrument;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    let router = Router::new()
        .route("/v1/listen", routing::any(handler))
        .route("/v1", routing::any(handler));

    let port = listener.local_addr().unwrap().port();
    let addr = listener.local_addr().unwrap().ip();
    tracing::info!("starting server at {addr}:{port}");

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

#[instrument]
async fn handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(DeepgramResponse {
            results: Results {
                channels: vec![Channel {
                    alernatives: vec![Alternative::default()],
                }],
            },
            metdata: Metadata::default(),
        }),
    )
        .into_response()
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct DeepgramResponse {
    pub results: Results,
    pub metdata: Metadata,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct Metadata {
    pub transaction_key: String,
    pub request_id: String,
    pub sha256: String,
    pub created: String,
    pub duration: f64,
    pub channels: usize,
    pub models: Vec<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            transaction_key: "transaction_key".into(),
            request_id: "request_id".into(),
            sha256: "sha256".into(),
            created: "created".into(),
            duration: 20.0,
            channels: 0,
            models: vec!["nova".into()],
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct Results {
    pub channels: Vec<Channel>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct Channel {
    pub alernatives: Vec<Alternative>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct Alternative {
    pub transcript: String,
    pub confidence: f64,
    pub words: Vec<Word>,
}

impl Default for Alternative {
    fn default() -> Self {
        Self {
            transcript: "lorem ipsum dolor sit amet".into(),
            confidence: 0.95,
            words: vec![
                Word::new("lorem"),
                Word::new("ipsum"),
                Word::new("dolor"),
                Word::new("sit"),
                Word::new("amet"),
            ],
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct Word {
    pub word: String,
    pub start: f64,
    pub end: f64,
    pub confidence: f64,
}

impl Word {
    fn new(word: &str) -> Self {
        Self {
            end: 1.1,
            start: 0.0,
            confidence: 2.0,
            word: word.into(),
        }
    }
}
