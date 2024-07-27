use self::models::Device;
use diesel::prelude::*;
use pyum::schema::devices::dsl::*;
use pyum::{models, schema::devices};

fn establish_connection() -> MysqlConnection {
    let url = std::env::var("DATABASE_URL").unwrap();
    MysqlConnection::establish(&url).unwrap()
}

fn main() {
    let connection = &mut establish_connection();
    let results = devices
        .limit(5)
        .select(Device::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for devise in results {
        println!("{}", devise.id);
        println!("-----------\n");
        println!("{}", devise.name);
    }
}
