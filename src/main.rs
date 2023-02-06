use crate::db::users::models::GitList;
use crate::db::users::schema::git_lists;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
struct GitListPagination {
    #[serde(default = "default_page_size")]
    pub page_size: i64,

    #[serde(default = "default_page_num")]
    pub page_num: i64,
}

fn default_page_size() -> i64 {
    5
}

fn default_page_num() -> i64 {
    0
}

#[get("/git/list")]
async fn hello(info: web::Query<GitListPagination>) -> Result<impl Responder> {
    use crate::db::users::schema::git_lists::dsl::is_deleted;
    let connection = &mut establish_connection();
    let users = git_lists::table
        .filter(is_deleted::eq(is_deleted, 0)) // 过滤掉已删除的
        .order(git_lists::id.asc()) // 按id升序
        .limit(info.page_size) // 每页5条
        .offset(info.page_num * info.page_size) // 页数
        .load::<GitList>(connection)
        .expect("Error loading users");

    Ok(web::Json(users))
}

#[derive(Deserialize)]
struct AddType {
    pub name: String,
    pub url: String,
}

#[post("/git/add")]
async fn add(info: web::Json<AddType>) -> Result<impl Responder> {
    let connection = &mut establish_connection();
    let new_id = git_lists::table
        .select(diesel::dsl::max(git_lists::id))
        .first::<Option<i32>>(connection)
        .expect("Error loading max id")
        .unwrap_or(0);
    let new_user = GitList {
        id: new_id,
        name: info.name.clone(),
        url: info.url.clone(),
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
