use crate::web::{middleware::AppState, SimpleRes};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use model_entity::models::{
    sensor_event,
    sensor_purpose::{self, query::SensorPurposeQuery},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Serialize)]
pub struct List {
    sensor_purposes: Vec<(
        sensor_purpose::model::Model,
        Option<sensor_event::model::Model>,
    )>,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub models_per_page: Option<u64>,
}

pub async fn list(
    session: Session,
    state: State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<List>, Json<SimpleRes>> {
    let uid = session.get("uid").await.unwrap().unwrap();
    let page = params.page.unwrap_or(1);
    let models_per_page = params.models_per_page.unwrap_or(5);
    let (sensor_purposes, _num_pages) =
        SensorPurposeQuery::find_in_page(&state.conn, uid, page, models_per_page)
            .await
            .map_err(|e| {
                println!("error: {e:?}");
                Json(SimpleRes {
                    message: "Cannot find sensor purposes in page".to_string(),
                })
            })?;

    Ok(Json(List { sensor_purposes }))
}

pub async fn create(
    session: Session,
    state: State<AppState>,
    Json(new_sensor_purpose): Json<sensor_purpose::model::Model>,
) -> Result<Json<SimpleRes>, Json<SimpleRes>> {
    let uid = session.get("uid").await.unwrap().unwrap();
    println!("uid: {uid:?}");
    sensor_purpose::mutation::create(&state.conn, new_sensor_purpose, uid)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Successed to create sensor purpose.".to_string(),
    }))
}

#[derive(Serialize)]
pub struct Detail {
    sensor_purpose: sensor_purpose::model::Model,
    sensor_event: sensor_event::model::Model,
}

pub async fn detail(
    state: State<AppState>,
    Path(sensor_purpose_id): Path<i32>,
) -> Result<Json<Detail>, (StatusCode, &'static str)> {
    let models = sensor_purpose::mutation::get_by_id(&state.conn, sensor_purpose_id)
        .await
        .unwrap();

    Ok(Json(Detail {
        sensor_purpose: models.0,
        sensor_event: models.1,
    }))
}

pub async fn edit(
    state: State<AppState>,
    Path(sensor_purpose_id): Path<i32>,
    Json(sensor_purpose): Json<sensor_purpose::model::Model>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    sensor_purpose::mutation::update_by_id(&state.conn, sensor_purpose_id, sensor_purpose)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Success to edit sensor_purpose".to_string(),
    }))
}

pub async fn delete(
    state: State<AppState>,
    Path(sensor_purpose_id): Path<i32>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    sensor_purpose::mutation::delete_by_id(&state.conn, sensor_purpose_id)
        .await
        .unwrap();

    Ok(Json(SimpleRes {
        message: "Success to delete sensor purpose".to_string(),
    }))
}
