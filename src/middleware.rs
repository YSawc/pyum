use axum::{
    http::HeaderValue,
    response::{IntoResponse, Redirect, Response},
};

use sea_orm::DatabaseConnection;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub templates: Tera,
    pub conn: DatabaseConnection,
}

impl AppState {
    pub fn new(conn: DatabaseConnection) -> Self {
        let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*"))
            .expect("Tera initialization failed");
        Self { templates, conn }
    }
}

pub fn error_response(code: u16, message: &str) -> Response {
    let to = format!("/error?code={}&message={}", code, message);
    let r = Redirect::to(&to);
    let mut r = r.into_response();
    let h = r.headers_mut();
    h.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
    r
}
