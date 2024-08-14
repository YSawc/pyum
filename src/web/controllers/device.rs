use crate::{
    flash::get_flash_cookie,
    web::middleware::AppState,
    web::routes::{FlashData, Params},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Redirect},
    Form,
};
use model_entity::device::{self, query::DeviceQuery};
use tower_cookies::Cookies;

pub async fn list_devices(
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

pub async fn new_device(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
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

pub async fn create_device(
    state: State<AppState>,
    Form(new_device): Form<device::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    device::mutation::create(&state.conn, new_device)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}

pub async fn detail_device(
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

pub async fn get_edit_device(
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

pub async fn post_edit_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
    Form(new_device): Form<device::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    device::mutation::update_by_id(&state.conn, device_id, new_device)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}

pub async fn delete_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    device::mutation::delete_by_id(&state.conn, device_id)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}
