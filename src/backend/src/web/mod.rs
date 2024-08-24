use serde::Serialize;

pub mod auth;
pub mod controllers;
pub mod middleware;
pub mod routes;

#[derive(Serialize)]
pub struct SimpleRes {
    message: String,
}
