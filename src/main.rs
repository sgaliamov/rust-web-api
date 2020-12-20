use actix_web::{
    get, post,
    web::{self, Json, Path},
    App, HttpResponse, HttpServer, Result,
};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_postgres::{
    postgres::{types::Timestamp, NoTls},
    PostgresConnectionManager,
};
use serde::Serialize;

#[derive(Serialize)]
struct Note {
    id: i32,
    text: String,
    timestamp: DateTime<Utc>,
}

#[get("/{id}")]
async fn get_note(
    Path(id): Path<i32>,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<Json<Note>> {
    let mut conn = db.get().unwrap();
    let result = conn.query_one("SELECT 'text'", &[]).unwrap();
    let value = result.get(0);

    Ok(Json(Note {
        id,
        text: value,
        timestamp: Utc::now(),
    }))
}

#[post("/")]
async fn add_note(
    text: String,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<Json<Note>> {
    // let res = web::block(move || {
    //     let conn = db.get().unwrap();

    //     let one = conn
    //         .query_one(
    //             "insert into notes (text) values ($1) returning id, timestamp",
    //             &[&text],
    //         )
    //         .unwrap();

    //     let id: u32 = one.get("id");
    //     let timestamp: String = one.get("timestamp");

    //     // Ok(Note {
    //     //     id,
    //     //     text: text.clone(),
    //     //     timestamp: Utc::now(),
    //     // })
    //     Note {
    //         id,
    //         text: text.clone(),
    //         timestamp: Utc::now(),
    //     }
    // })
    // .await
    // .map(|x| Json(x))
    // .map_err(|_| HttpResponse::InternalServerError())?;
    // Ok(res)

    let mut conn = db.get().unwrap();

    let one = conn
        .query_one(
            "insert into notes (text) values ($1) returning id, timestamp",
            &[&text],
        )
        .unwrap();

    let id: i32 = one.get("id");
    let timestamp: DateTime<Utc> = one.get("timestamp");

    Ok(Json(Note {
        id,
        text: text.clone(),
        timestamp,
    }))
}

// todo:
// + 1. 2 endpoints
// + 2. deserialize post
// + 3. serialize get
// + 4. connect to db https://docs.rs/tokio-postgres/0.6.0/tokio_postgres/index.html
// + 5. write to db
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
                timestamp timestamptz default now() not null
            )",
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
