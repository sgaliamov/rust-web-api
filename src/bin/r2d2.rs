use actix_web::{
    get, post,
    web::{self, Path},
    App, Error, HttpResponse, HttpServer, Result,
};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
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
) -> Result<HttpResponse, Error> {
    let res = web::block::<_, _, r2d2_postgres::postgres::Error>(move || {
        let mut conn = db.get().unwrap();

        let one = conn.query_one(
            "select id, text, timestamp
                from notes
                where id = $1",
            &[&id],
        )?;

        let id: i32 = one.get("id");
        let text: String = one.get("text");
        let timestamp: DateTime<Utc> = one.get("timestamp");

        Ok(Note {
            id,
            text,
            timestamp,
        })
    })
    .await
    .map(|x| HttpResponse::Ok().json(x))
    .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(res)
}

#[post("/")]
async fn add_note(
    text: String,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<HttpResponse, Error> {
    let res = web::block::<_, _, r2d2_postgres::postgres::Error>(move || {
        let mut conn = db.get().unwrap();

        let one = conn.query_one(
            "insert into notes (text) values ($1) returning id, timestamp",
            &[&text],
        )?;

        let id: i32 = one.get("id");
        let timestamp: DateTime<Utc> = one.get("timestamp");

        Ok(Note {
            id,
            text,
            timestamp,
        })
    })
    .await
    .map(|x| HttpResponse::Ok().json(x))
    .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(res)
}

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

    println!("Listening on http://localhost:8080");

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
