use actix_web::{get, App, HttpServer, Responder};
use rusqlite::Connection;
// use rusqlite::{params, Connection, Result};

#[get("/")]
async fn index() -> impl Responder {
    //let conn = Connection::open("test.db").unwrap();
    //let mut stmt = conn.prepare("SELECT name FROM users").unwrap();
    /*let rows = stmt.query_map([], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    }).unwrap();

    let mut names = Vec::new();
    for row in rows {
        names.push(row.unwrap());
    }*/

    let response = "Users: {:?}";
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}