mod models;
mod schema;

use diesel::PgConnection;
use models::Url;

use crate::models::{create_url, establish_connection, get_url_by_key};

#[macro_use]
extern crate diesel;

fn main() {
    let conn: PgConnection = establish_connection();

    // let url: Url = create_url(
    //     &conn,
    //     "geeksesi".to_string(),
    //     "http://geeksesi.ir".to_string(),
    // );

    let key: String = "geeksesi".to_string();
    let url: Url = get_url_by_key(&conn, &key);
    println!("{:?}", url);
}
