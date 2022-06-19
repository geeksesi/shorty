mod models;
mod schema;

use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use models::{NewUrl, Url};
use schema::*;
use std::env;

#[macro_use]
extern crate diesel;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_url(conn: &PgConnection, key: String, url: String) -> Url {
    let new_url = NewUrl { key: key, url: url };

    diesel::insert_into(urls::table)
        .values(&new_url)
        .get_result(conn)
        .expect("Error saving new short url")
}

fn main() {
    let conn: PgConnection = establish_connection();

    let url: Url = create_url(
        &conn,
        "geeksesi".to_string(),
        "http://geeksesi.ir".to_string(),
    );

    println!("{:?}", url);
}
