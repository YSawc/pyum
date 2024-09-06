use crate::web::{middleware::AppState, SimpleRes};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use model_entity::models::device::{self, query::DeviceQuery};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Serialize)]
pub struct ListDevices {
    devices: Vec<device::model::Model>,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub models_per_page: Option<u64>,
}

pub async fn list_devices(
    session: Session,
    state: State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<ListDevices>, Json<SimpleRes>> {
    let uid = session.get("uid").await.unwrap().unwrap();
    let page = params.page.unwrap_or(1);
    let models_per_page = params.models_per_page.unwrap_or(5);
    let (devices, _num_pages) = DeviceQuery::find_in_page(&state.conn, uid, page, models_per_page)
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
        message: "Successed to create device.".to_string(),
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
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    device::mutation::update_by_id(&state.conn, device_id, new_device)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Success to edit device".to_string(),
    }))
}

pub async fn delete_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    device::mutation::delete_by_id(&state.conn, device_id)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Success to delete device".to_string(),
    }))
}
