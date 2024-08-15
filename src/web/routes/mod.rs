pub mod protected;
pub mod published;

use axum::{
    extract::{MatchedPath, Request},
    middleware, Router,
};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

use crate::web::middleware::{print_request_response, AppState};

#[derive(Deserialize)]
pub struct Params {
    pub page: Option<u64>,
    pub devices_per_page: Option<u64>,
}

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
    let state = AppState::new(conn);
    Router::new()
        .merge(protected::router())
        .merge(published::router())
        .layer(CookieManagerLayer::new())
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
        .layer(middleware::from_fn(print_request_response))
        .with_state(state)
}
