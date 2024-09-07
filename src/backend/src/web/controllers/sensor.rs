use crate::web::{middleware::AppState, SimpleRes};
use axum::{
    extract::{Path, Query, State},
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
    let sensors_and_relationed_purpose =
        SensorQuery::find_in_page(&state.conn, device_id, page, models_per_page)
            .await
            .map_err(|_| {
                Json(SimpleRes {
                    message: "Cannot find sensors in page".to_string(),
                })
            })?;

    let models = sensors_and_relationed_purpose
        .iter()
        .map(|rel| (rel.0.to_owned(), rel.1.first().unwrap().to_owned()))
        .collect::<Vec<_>>();

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
