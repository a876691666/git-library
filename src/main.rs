use crate::db::users::models::User;
use crate::db::users::schema::users;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub mod db;

fn establish_connection() -> SqliteConnection {
    let database_url = "./data/users.db";
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn random_string() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

#[get("/")]
async fn hello() -> impl Responder {
    let connection = &mut establish_connection();
    let users = users::table
        .load::<User>(connection)
        .expect("Error loading users");
    let string = users
        .iter()
        .map(|user| {
            format!(
                "{}: {}: {}: {}",
                user.id, user.name, user.password, user.email
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    HttpResponse::Ok().body(string)
}

#[get("/add")]
async fn add() -> Result<impl Responder> {
    let connection = &mut establish_connection();
    let new_id = users::table
        .select(diesel::dsl::max(users::id))
        .first::<Option<i32>>(connection)
        .expect("Error loading max id")
        .unwrap_or(0)
        + 1;
    let new_user = User {
        id: new_id,
        name: "test".to_string(),
        email: random_string(),
        password: random_string(),
    };
    diesel::insert_into(users::table)
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
