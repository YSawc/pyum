use crate::web::{middleware::AppState, SimpleRes};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use model_entity::models::{
    sensor::{self, query::SensorQuery},
    sensor_purpose,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ListRelatedSensor {
    models: Vec<(sensor::model::Model, sensor_purpose::model::Model)>,
}

#[derive(Deserialize)]
pub struct ListRelatedSensorParams {
    pub page: Option<u64>,
    pub models_per_page: Option<u64>,
}

pub async fn list_related_sensor(
    state: State<AppState>,
    Path(device_id): Path<i32>,
    Query(params): Query<ListRelatedSensorParams>,
) -> Result<Json<ListRelatedSensor>, Json<SimpleRes>> {
    let page = params.page.unwrap_or(1);
    let models_per_page = params.models_per_page.unwrap_or(5);
    let models = SensorQuery::find_in_page(&state.conn, device_id, page, models_per_page)
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
    Path(device_id): Path<i32>,
    Json(new_sensor_purpose): Json<sensor::model::Model>,
) -> Result<Json<SimpleRes>, Json<SimpleRes>> {
    sensor::mutation::create(&state.conn, new_sensor_purpose, device_id)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Successed to create sensor.".to_string(),
    }))
}

#[derive(Serialize)]
pub struct Detail {
    models: (sensor::model::Model, sensor_purpose::model::Model),
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
