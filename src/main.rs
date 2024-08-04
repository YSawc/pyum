use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use model_entity::{device, device::model::Entity as Device, device::query::DeviceQuery};
use pyum::{
    flash::{get_flash_cookie, post_response, PostResponse},
    middleware::AppState,
};
use std::net::SocketAddr;
use tera::Tera;
use tower_cookies::{CookieManagerLayer, Cookies};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params {
    page: Option<u64>,
    devices_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tokio_mysql=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let conn = Database::connect(db_url)
        .await
        .expect("database connection failed.");

    let state = AppState::new(conn);
    let app = Router::new()
        .route("/api/health_check", get(health_check_handler))
        .route("/hello", get(hello))
        // .route("/devices/list", get(list_devices))
        // .route("/device/list", post(create_device))
        .layer(CookieManagerLayer::new())
        // .layer(axum::middleware::from_fn_with_state(
        //     state.clone(),
        //     handle_error,
        // ))
        .with_state(state);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("something is go wrong when axum server is launching");

    Ok(())
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

async fn hello(state: State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
    const MESSAGE: &str = "Hello from tera";

    let mut ctx = tera::Context::new();
    // let body = state.templates.render("index.html.tera", &ctx);

    let body = state
        .templates
        .render("pages/hello.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

// async fn list_devices(
//     state: State<AppState>,
//     Query(params): Query<Params>,
//     cookies: Cookies,
// ) -> Result<Html<String>, (StatusCode, &'static str)> {
//     let page = params.page.unwrap_or(1);
//     let devices_per_page = params.devices_per_page.unwrap_or(5);
//
//     let (devices, num_pages) = DeviceQuery::find_device_by_id(&state.conn, page, devices_per_page)
//         .await
//         .map_err(|_| (StatusCode::OK, "Cannot find devices in page"))?;
//
//     let mut ctx = tera::Context::new();
//     ctx.insert("devices", &devices);
//     ctx.insert("page", &page);
//     ctx.insert("devices_per_page", &devices_per_page);
//     ctx.insert("num_pages", &num_pages);
//
//     if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
//         ctx.insert("flash", &value);
//     }
//
//     let body = state
//         .templates
//         .render("index.html", &ctx)
//         .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;
//
//     Ok(Html(body))
// }

// async fn create_device(
//     State(pool): State<deadpool_diesel::mysql::Pool>,
//     Json(new_devise): Json<NewDevice>,
// ) -> Result<impl IntoResponse, (StatusCode, String)> {
//     // ) -> Result<Json<Device>, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(internal_error)?;
//     let res = conn
//         .interact(|conn| {
//             diesel::insert_into(devices::table)
//                 .values(new_devise)
//                 .execute(conn)
//                 .unwrap();
//             // .get_result(conn)
//         })
//         .await
//         .map_err(internal_error)?;
//
//     let json_response = serde_json::json!({
//         "status": "ok",
//         "message": "insert successed."
//     });
//
//     Ok(Json(json_response))
// }

// Utility function for mapping any error into a `500 Internal Server Error`
// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
