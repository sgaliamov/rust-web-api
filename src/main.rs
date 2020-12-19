use actix_web::{
    get, post,
    web::{Json, Path},
    App, HttpServer, Result,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct Note {
    id: u32,
    text: String,
    timestamp: DateTime<Utc>,
}

#[get("/{id}")]
async fn get_note(Path(id): Path<u32>) -> Result<Json<Note>> {
    Ok(Json(Note {
        id,
        text: "text".to_string(),
        timestamp: Utc::now(),
    }))
}

#[post("/")]
async fn add_note(text: String) -> Result<Json<u32>> {
    Ok(Json(1))
}

// todo:
// + 1. 2 endpoints
// 2. deserialize post
// + 3. serialize get
// 4. connect to db https://docs.rs/tokio-postgres/0.6.0/tokio_postgres/index.html
// 5. write to db
// 6. query from db
// 7. c# version
// 8. benchmark

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_note).service(add_note))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
