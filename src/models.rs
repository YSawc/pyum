// use diesel::prelude::*;
//
// use crate::schema;
//
// #[derive(serde::Serialize, Queryable, Selectable)]
// #[diesel(table_name = schema::devices)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
// pub struct Device {
//     pub id: i32,
//     pub name: String,
// }
//
// #[derive(serde::Deserialize, Insertable)]
// #[diesel(table_name = schema::devices)]
// pub struct NewDevice {
//     name: String,
// }
