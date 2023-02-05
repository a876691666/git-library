use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

fn establish_connection() -> SqliteConnection {
    let database_url = "sqlite:///tmp/test.db";
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    let connection = &mut establish_connection();
    let users = users::table.load::<User>(connection).expect("Error loading users");
    let string = users.iter().map(|user| format!("{}: {}", user.id, user.name)).collect::<Vec<String>>().join("\n");
    HttpResponse::Ok().body(string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}