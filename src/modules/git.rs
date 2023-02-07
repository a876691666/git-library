use crate::db::users::models::GitList;
use crate::db::users::schema::git_lists;
use actix_web::{get, post, web, Responder, Result};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::Deserialize;

fn establish_connection() -> SqliteConnection {
    let database_url = "./data/sqlite.db";
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_utc_time() -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp()
}

#[derive(Deserialize, Debug)]
struct GitListPagination {
    #[serde(default = "default_page_size")]
    pub page_size: i64,

    #[serde(default = "default_num")]
    pub page_num: i64,
}

fn default_page_size() -> i64 {
    5
}

fn default_num() -> i64 {
    0
}

fn default_empty_string() -> String {
    "".to_string()
}

#[get("/list")]
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

/// 添加git项目的请求参数
#[derive(Deserialize, Debug)]
struct AddType {
    /// Git项目名称
    pub name: String,

    /// Git项目地址
    pub url: String,

    /// 描述
    #[serde(default = "default_empty_string")]
    pub description: String,

    /// 标签
    #[serde(default = "default_empty_string")]
    pub tags: String,

    /// Git项目信息
    #[serde(default = "default_empty_string")]
    pub info: String,

    /// Git项目信息更新时间
    #[serde(default = "get_utc_time")]
    pub info_updated_at: i64,

    /// 创建时间
    #[serde(default = "get_utc_time")]
    pub created_at: i64,

    /// 更新时间
    #[serde(default = "get_utc_time")]
    pub updated_at: i64,

    /// 是否删除, 0: 未删除, 1: 已删除
    #[serde(default = "default_num")]
    pub is_deleted: i64,
}

#[post("/add")]
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
        description: info.description.clone(),
        tags: info.tags.clone(),
        is_deleted: info.is_deleted as i32,
        created_at: info.created_at as i32,
        updated_at: info.updated_at as i32,
        info: info.info.clone(),
        info_updated_at: info.info_updated_at as i32,
    };
    diesel::insert_into(git_lists::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new post");

    Ok(web::Json(new_user))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/git").service(hello).service(add));
}
