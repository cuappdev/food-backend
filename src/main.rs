use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
            .configure(user::init) // init routes
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
