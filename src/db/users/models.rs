use diesel::prelude::*;
use super::schema::users;
use serde::Serialize;

#[derive(Queryable, Insertable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
