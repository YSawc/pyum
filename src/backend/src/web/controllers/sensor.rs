use crate::web::{middleware::AppState, SimpleRes};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use model_entity::models::sensor::{self, query::SensorQuery};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ListRelatedDevice {
    sensors: Vec<sensor::model::Model>,
}

#[derive(Deserialize)]
pub struct ListRelatedDeviceParams {
    pub page: Option<u64>,
    pub models_per_page: Option<u64>,
}

pub async fn list_related_device(
    state: State<AppState>,
    Path(device_id): Path<i32>,
    Query(params): Query<ListRelatedDeviceParams>,
) -> Result<Json<ListRelatedDevice>, Json<SimpleRes>> {
    let page = params.page.unwrap_or(1);
    let models_per_page = params.models_per_page.unwrap_or(5);
    let (sensors, _num_pages) =
        SensorQuery::find_in_page(&state.conn, device_id, page, models_per_page)
            .await
            .map_err(|_| {
                Json(SimpleRes {
                    message: "Cannot find sensor purposes in page".to_string(),
                })
            })?;

    Ok(Json(ListRelatedDevice { sensors }))
}
