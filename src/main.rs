use crate::db::users::models::GitList;
use crate::db::users::schema::git_lists;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub mod db;

fn establish_connection() -> SqliteConnection {
    let database_url = "./data/sqlite.db";
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_utc_time() -> i32 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() as i32
}

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let connection = &mut establish_connection();
    let users = git_lists::table
        .load::<GitList>(connection)
        .expect("Error loading users");
    Ok(web::Json(users))
}

#[get("/add")]
async fn add() -> Result<impl Responder> {
    let connection = &mut establish_connection();
    let new_id = git_lists::table
        .select(diesel::dsl::max(git_lists::id))
        .first::<Option<i32>>(connection)
        .expect("Error loading max id")
        .unwrap_or(0)
        + 1;
    let new_user = GitList {
        id: new_id,
        name: "test".to_string(),
        url: "".to_string(),
        description: "".to_string(),
        tags: "".to_string(),
        is_deleted: 0,
        created_at: get_utc_time(),
        updated_at: get_utc_time(),
        info: "{}".to_string(),
        info_updated_at: get_utc_time(),
    };
    diesel::insert_into(git_lists::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new post");
    Ok(web::Json(new_user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(add))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
