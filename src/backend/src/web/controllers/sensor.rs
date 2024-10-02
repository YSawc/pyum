use crate::web::{middleware::AppState, SimpleRes};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use model_entity::models::{
    sensor::{
        self,
        query::{FindInPageResult, SensorQuery},
    },
    sensor_event, sensor_purpose,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ListRelatedSensor {
    models: Vec<FindInPageResult>,
}

#[derive(Deserialize)]
pub struct ListRelatedSensorParams {
    pub page: Option<u64>,
    pub models_per_page: Option<u64>,
    pub device_id: Option<i32>,
}

pub async fn list_related_sensor(
    state: State<AppState>,
    Query(params): Query<ListRelatedSensorParams>,
) -> Result<Json<ListRelatedSensor>, Json<SimpleRes>> {
    let page = params.page.unwrap_or(1);
    let models_per_page = params.models_per_page.unwrap_or(20);
    let models = SensorQuery::find_devices_with_related_sensor_and_purpose(
        &state.conn,
        page,
        models_per_page,
        params.device_id,
    )
    .await
    .map_err(|_| {
        Json(SimpleRes {
            message: "Cannot find sensors in page".to_string(),
        })
    })?;

    Ok(Json(ListRelatedSensor { models }))
}

pub async fn create(
    state: State<AppState>,
    Json(new_sensor_purpose): Json<sensor::model::Model>,
) -> Result<Json<SimpleRes>, Json<SimpleRes>> {
    sensor::mutation::create(&state.conn, new_sensor_purpose)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Successed to create sensor.".to_string(),
    }))
}

#[derive(Serialize)]
pub struct Detail {
    models: (
        sensor::model::Model,
        sensor_purpose::model::Model,
        sensor_event::model::Model,
    ),
}

pub async fn detail(
    state: State<AppState>,
    Path(sensor_id): Path<i32>,
) -> Result<Json<Detail>, (StatusCode, &'static str)> {
    let models = sensor::mutation::get_by_id(&state.conn, sensor_id)
        .await
        .unwrap();

    Ok(Json(Detail { models }))
}

pub async fn edit(
    state: State<AppState>,
    Path(sensor_id): Path<i32>,
    Json(new_sensor_purpose): Json<sensor::model::Model>,
) -> Result<Json<SimpleRes>, Json<SimpleRes>> {
    sensor::mutation::update(&state.conn, new_sensor_purpose, sensor_id)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Successed to create sensor.".to_string(),
    }))
}

pub async fn delete(
    state: State<AppState>,
    Path(sensor_id): Path<i32>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    sensor::mutation::delete_by_id(&state.conn, sensor_id)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Success to delete sensor".to_string(),
    }))
}
