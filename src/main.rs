use actix_web::{
    get, post,
    web::{self, Json, Path},
    App, HttpServer, Result,
};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use serde::Serialize;

#[derive(Serialize)]
struct Note {
    id: u32,
    text: String,
    timestamp: DateTime<Utc>,
}

#[get("/{id}")]
async fn get_note(
    Path(id): Path<u32>,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<Json<Note>> {
    let mut client = db.get().unwrap();
    let result = client.query_one("SELECT 'text'", &[]).unwrap();
    let value = result.get(0);

    Ok(Json(Note {
        id,
        text: value,
        timestamp: Utc::now(),
    }))
}

#[post("/")]
async fn add_note(text: String) -> Result<Json<u32>> {
    Ok(Json(1))
}

// todo:
// + 1. 2 endpoints
// + 2. deserialize post
// + 3. serialize get
// + 4. connect to db https://docs.rs/tokio-postgres/0.6.0/tokio_postgres/index.html
// 5. write to db
// 6. query from db
// 7. c# version
// 8. benchmark

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres port=5432 password=postgres"
            .parse()
            .unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();
    let mut client = pool.get().unwrap();
    client
        .execute(
            "create table if not exists notes (
                id serial,
                text varchar not null,
                timestamp timestamp without time zone default (now() at time zone 'utc'))",
            &[],
        )
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(get_note)
            .service(add_note)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
