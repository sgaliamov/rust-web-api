#![feature(async_closure)]

use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Path},
    App, Error, HttpResponse, HttpServer, Result,
};
use bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};
use chrono::{DateTime, Utc};
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
    match db
        .get_ref()
        .run(async move |cl| {
            let prepare = cl
                .prepare(
                    "select id, text, timestamp
                    from notes
                    where id = $1",
                )
                .await;

            match prepare {
                Ok(select) => match cl.query_one(&select, &[&id]).await {
                    Ok(one) => {
                        let id: i32 = one.get("id");
                        let text: String = one.get("text");
                        let timestamp: DateTime<Utc> = one.get("timestamp");
                        let note = Note {
                            id,
                            text,
                            timestamp,
                        };
                        Ok((note, cl))
                    }
                    Err(e) => Err((e, cl)),
                },
                Err(e) => Err((e, cl)),
            }
        })
        .await
    {
        Ok(x) => Ok(HttpResponse::Ok().json(x)),
        Err(e) => {
            println!("{}", e);
            Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

#[post("/")]
async fn add_note(
    text: String,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<HttpResponse, Error> {
    match db
        .get_ref()
        .run(async move |cl| {
            let prepare = cl
                .prepare("insert into notes (text) values ($1) returning id, timestamp")
                .await;

            match prepare {
                Ok(select) => match cl.query_one(&select, &[&text]).await {
                    Ok(one) => {
                        let id: i32 = one.get("id");
                        let timestamp: DateTime<Utc> = one.get("timestamp");
                        let note = Note {
                            id,
                            text,
                            timestamp,
                        };
                        Ok((note, cl))
                    }
                    Err(e) => Err((e, cl)),
                },
                Err(e) => Err((e, cl)),
            }
        })
        .await
    {
        Ok(x) => Ok(HttpResponse::Ok().json(x)),
        Err(e) => {
            println!("{}", e);
            Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres port=5432 password=postgres"
            .parse()
            .unwrap(),
        NoTls,
    );
    let pool = Pool::builder().build(manager).await.unwrap();
    let client = pool.get().await.unwrap();
    client
        .execute(
            "create table if not exists notes (
                id serial,
                text varchar not null,
                timestamp timestamptz default now() not null
            )",
            &[],
        )
        .await
        .unwrap();

    let pool = pool.clone();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(get_note)
            .service(add_note)
    })
    .bind("127.0.0.1:9080")?
    .run()
    .await
}
