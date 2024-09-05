pub mod protected;
pub mod published;

use axum::{
    extract::{MatchedPath, Request},
    middleware, Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use time::Duration;
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use crate::web::middleware::{print_request_response, AppState};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FlashKind {
    Error,
    Info,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    pub kind: FlashKind,
    pub message: String,
}

pub async fn router(conn: DatabaseConnection) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let state = AppState::new(conn);
    Router::new()
        .merge(protected::router(state.clone()))
        .merge(published::router())
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();
                    // axum automatically adds this extension.
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        )
        .layer(session_layer)
        .layer(middleware::from_fn(print_request_response))
}
