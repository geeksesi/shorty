use super::schema::urls;
use diesel::{Connection, PgConnection, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;

#[derive(Queryable, Debug)]
pub struct Url {
    pub id: i32,
    pub key: String,
    pub url: String,
    pub created_at: SystemTime,
}

#[derive(Insertable, Debug)]
#[table_name = "urls"]
pub struct NewUrl {
    pub key: String,
    pub url: String,
}

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

pub fn get_url_by_key(conn: &PgConnection, _key: &String) -> Url {
    use super::schema::urls::dsl::*;
    use crate::diesel::ExpressionMethods;

    urls.filter(key.eq(_key))
        .limit(1)
        .first::<Url>(conn)
        .expect("Error loading url")
}
