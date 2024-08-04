use sea_orm::*;

use super::{model, model as Oauth2ClientSecret};

pub async fn create_device(
    db: &DbConn,
    form_data: Oauth2ClientSecret::Model,
) -> Result<model::ActiveModel, DbErr> {
    Oauth2ClientSecret::ActiveModel {
        ..Default::default()
    }
    .save(db)
    .await
}

// pub async fn update_device_by_id(
//     db: &DbConn,
//     id: i32,
//     form_data: model::Model,
// ) -> Result<model::Model, DbErr> {
//     let device: model::ActiveModel = Oauth2ClientSecret::find_by_id(id)
//         .one(db)
//         .await?
//         .ok_or(DbErr::Custom("Cannot find device.".to_owned()))
//         .map(Into::into)?;
//
//     model::ActiveModel {
//         id: device.id,
//         // name: Set(form_data.name.to_owned()),
//     }
//     .update(db)
//     .await
// }
//
// pub async fn delete_device(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
//     let device: model::ActiveModel = Oauth2ClientSecret::find_by_id(id)
//         .one(db)
//         .await?
//         .ok_or(DbErr::Custom("Cannot find device.".to_owned()))
//         .map(Into::into)?;
//
//     device.delete(db).await
// }
//
// pub async fn delete_all_devices(db: &DbConn) -> Result<DeleteResult, DbErr> {
//     Device::delete_many().exec(db).await
// }
