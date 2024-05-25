use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::path::Path;
use tokio::time::{sleep, Duration};

#[derive(Deserialize)]
struct InputData {
    text: String,
    array: Vec<i32>,
}

fn cyclic_shift_string(s: &str) -> String {
    if s.is_empty() {
        return s.to_string();
    }
    let mut chars: Vec<char> = s.chars().collect();
    let last = chars.pop().unwrap();
    let mut shifted_string = String::new();
    shifted_string.push(last);
    shifted_string.extend(chars);
    shifted_string
}

fn cyclic_shift_array(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return arr.to_vec();
    }
    let mut shifted_array = arr.to_vec();
    let last = shifted_array.pop().unwrap();
    let mut result = vec![last];
    result.extend(shifted_array);
    result
}

#[post("/")]
async fn index(item: web::Json<InputData>) -> impl Responder {
    let shifted_text = cyclic_shift_string(&item.text);
    let shifted_array = cyclic_shift_array(&item.array);

    let db_path = Path::new("../SQL/messages.db");
    let mut conn = Connection::open(db_path).unwrap();

    let tx = conn.transaction().unwrap();
    tx.execute(
        "INSERT INTO InputMessages (string, arraySize) VALUES (?1, ?2)",
        params![shifted_text, shifted_array.len() as i32],
    ).unwrap();

    let id: i64 = tx.last_insert_rowid();

    let mut stmt = String::from("INSERT INTO ArrayElements (messageId, arrayIndex, intValue) VALUES ");
    for (index, value) in shifted_array.iter().enumerate() {
        stmt.push_str(&format!("({}, {}, {}),", id, index, value));
    }
    stmt.pop();
    tx.execute(&stmt, []).unwrap();
    tx.commit().unwrap();

    loop {
        let mut stmt = conn.prepare("SELECT string FROM OutputMessages WHERE id = ?1").unwrap();
        let mut rows = stmt.query(params![id]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            let result: String = row.get(0).unwrap();
            return HttpResponse::Ok().body(result);
        } else {
            sleep(Duration::from_millis(100)).await;
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .service(index)
    })
    .bind(("127.0.0.1", 8101))?
    .run()
    .await
}