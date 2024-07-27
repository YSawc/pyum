use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::devices)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Device {
    pub id: i32,
    pub name: String,
}
