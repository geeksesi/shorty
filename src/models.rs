use std::time::SystemTime;

use super::schema::*;

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
