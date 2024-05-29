use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::env;
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
async fn index(item: web::Json<InputData>, db_path: web::Data<PathBuf>) -> impl Responder {
    let shifted_text = cyclic_shift_string(&item.text);
    let shifted_array = cyclic_shift_array(&item.array);

    if !db_path.exists() {
        println!("Database file does not exist: {:?}", db_path);
        return HttpResponse::InternalServerError().body("Database file not found");
    }

    let mut conn = match Connection::open(db_path.get_ref()) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to open database: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to open database");
        }
    };

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_db>", args[0]);
        std::process::exit(1);
    }

    let db_relative_path = &args[1];
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let db_path = current_dir.join(db_relative_path);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .app_data(web::Data::new(db_path.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8101))?
    .run()
    .await
}
