#[macro_use]
extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::postgres::PgPool;
use std::env;

// import user module (routes and model)
mod user;

// / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Available routes:
        GET /users/ -> list of all users
        POST /user/ -> create new user, example: { "id": 1, "first_name": "Shungo", "last_name": "Najima" }
        GET /user/{id}/ -> show one user with requested id
        PUT /user/{id}/ -> update user with requested id, example: { "id": 1, "first_name": "Sarasa", "last_name": "Najima" }
        DELETE /user/{id} -> delete user with requested id
    "#
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
            .configure(user::init) // init routes
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}
