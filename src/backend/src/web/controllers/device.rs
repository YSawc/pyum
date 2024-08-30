use crate::web::{middleware::AppState, routes::Params, SimpleRes};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Redirect,
    Json,
};
use model_entity::models::device::{self, query::DeviceQuery};
use serde::Serialize;
use tower_sessions::Session;

#[derive(Serialize)]
pub struct ListDevices {
    devices: Vec<device::model::Model>,
}

pub async fn list_devices(
    session: Session,
    state: State<AppState>,
    Query(params): Query<Params>,
) -> Result<Json<ListDevices>, Json<SimpleRes>> {
    let uid = session.get("uid").await.unwrap().unwrap();
    let page = params.page.unwrap_or(1);
    let devices_per_page = params.devices_per_page.unwrap_or(5);
    let (devices, _num_pages) = DeviceQuery::find_in_page(&state.conn, uid, page, devices_per_page)
        .await
        .map_err(|_| {
            Json(SimpleRes {
                message: "Cannot find devices in page".to_string(),
            })
        })?;

    Ok(Json(ListDevices { devices }))
}

pub async fn create_device(
    session: Session,
    state: State<AppState>,
    Json(new_device): Json<device::model::Model>,
) -> Result<Json<SimpleRes>, Json<SimpleRes>> {
    let uid = session.get("uid").await.unwrap().unwrap();
    device::mutation::create(&state.conn, new_device, uid)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Successed to creat device.".to_string(),
    }))
}

#[derive(Serialize)]
pub struct DeviceDetail {
    device: device::model::Model,
}

pub async fn detail_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
) -> Result<Json<DeviceDetail>, (StatusCode, &'static str)> {
    let device = device::mutation::get_by_id(&state.conn, device_id)
        .await
        .unwrap()
        .unwrap();

    Ok(Json(DeviceDetail { device }))
}

pub async fn edit_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
    Json(new_device): Json<device::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    device::mutation::update_by_id(&state.conn, device_id, new_device)
        .await
        .unwrap();

    Ok(Redirect::to("/device"))
}

pub async fn delete_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    device::mutation::delete_by_id(&state.conn, device_id)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Success to delet device".to_string(),
    }))
}
