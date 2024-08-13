use axum::{
    body::Body,
    extract::{MatchedPath, Path, Query, Request, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware,
    response::{Html, IntoResponse, Json, Redirect},
    routing::{get, post},
    Form, Router,
};
use model_entity::{
    admin_user,
    device::{self, query::DeviceQuery},
};
use pyum::{
    flash::get_flash_cookie,
    middleware::{print_request_response, AppState},
};
use std::net::SocketAddr;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::trace::TraceLayer;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sea_orm::Database;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params {
    page: Option<u64>,
    devices_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum FlashKind {
    Error,
    Info,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: FlashKind,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let file_appender = RollingFileAppender::new(Rotation::DAILY, "/var/log/pyum", "logfile.log");
    // let subscriber = tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "tokio_mysql=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let conn = Database::connect(db_url)
        .await
        .expect("database connection failed.");

    let login_required_routes = Router::new()
        .route("/device/", get(list_devices))
        .route("/device/new", get(new_device))
        .route("/device/new", post(create_device))
        .route("/device/:device_id", get(detail_device))
        .route("/device/:device_id/edit", get(get_edit_device))
        .route("/device/:device_id/edit", post(post_edit_device))
        .route("/device/:device_id/delete", get(delete_device));
    let state = AppState::new(conn);
    let app = Router::new()
        .route("/api/health_check", get(health_check_handler))
        .route("/hello", get(hello))
        .route("/assets/images/:path", get(get_image_asset))
        .route("/admin_user/new", get(get_create_admin_user))
        .route("/admin_user/new", post(post_create_admin_user))
        .route("/admin_user/login", get(get_login_admin_user))
        .route("/admin_user/login", post(post_login_admin_user))
        .merge(login_required_routes)
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
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/hello.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

pub async fn get_image_asset(Path(path): Path<String>) -> impl IntoResponse {
    let image_path = format!("src/assets/images/{}", path);
    let file = match tokio::fs::File::open(image_path.to_owned()).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let content_type = match mime_guess::from_path(&image_path).first_raw() {
        Some(mime) => mime,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "MIME Type couldn't be determined".to_string(),
            ))
        }
    };
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headermap = HeaderMap::new();
    headermap.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).expect("header content type must be string."),
    );
    headermap.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str("inline").expect("header content of diposition must be string."),
    );

    Ok((headermap, body))
}

async fn list_devices(
    state: State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let page = params.page.unwrap_or(1);
    let devices_per_page = params.devices_per_page.unwrap_or(5);

    let (devices, num_pages) = DeviceQuery::find_in_page(&state.conn, page, devices_per_page)
        .await
        .map_err(|_| (StatusCode::OK, "Cannot find devices in page"))?;

    let mut ctx = tera::Context::new();
    ctx.insert("devices", &devices);
    ctx.insert("page", &page);
    ctx.insert("devices_per_page", &devices_per_page);
    ctx.insert("num_pages", &num_pages);
    // ctx.insert(
    //     "flash",
    //     &FlashData {
    //         kind: FlashKind::Info,
    //         message: "created device".to_string(),
    //     },
    // );

    if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
        ctx.insert("flash", &value);
    }

    let body = state
        .templates
        .render("pages/device/index.html", &ctx)
        .map_err(|e| {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error")
        })?;

    Ok(Html(body))
}

async fn new_device(state: State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/device/new.html", &ctx)
        .map_err(|e| {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error")
        })?;

    Ok(Html(body))
}

async fn create_device(
    state: State<AppState>,
    Form(new_device): Form<device::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    device::mutation::create(&state.conn, new_device)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}

async fn detail_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let device = device::mutation::get_by_id(&state.conn, device_id)
        .await
        .unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("device", &device);
    let body = state
        .templates
        .render("pages/device/detail.html", &ctx)
        .map_err(|e| {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error")
        })?;

    Ok(Html(body))
}

async fn get_edit_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let device = device::mutation::get_by_id(&state.conn, device_id)
        .await
        .unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("device", &device);
    let body = state
        .templates
        .render("pages/device/edit.html", &ctx)
        .map_err(|e| {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error")
        })?;

    Ok(Html(body))
}

async fn post_edit_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
    Form(new_device): Form<device::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    device::mutation::update_by_id(&state.conn, device_id, new_device)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}

async fn delete_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    device::mutation::delete_by_id(&state.conn, device_id)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}

async fn get_create_admin_user(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/admin_user/new.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn post_create_admin_user(
    state: State<AppState>,
    Form(admin_user): Form<admin_user::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    admin_user::mutation::create(&state.conn, admin_user)
        .await
        .expect("failed to create admin user.");

    Ok(Redirect::to("/device/"))
}

async fn get_login_admin_user(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/admin_user/login.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn post_login_admin_user(
    state: State<AppState>,
    Form(admin_user): Form<admin_user::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    let admin_user = admin_user::mutation::find_by_name(&state.conn, admin_user)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}
