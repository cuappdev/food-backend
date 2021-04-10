use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use std::env;
use std::io::ErrorKind;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = Pool::<Postgres>::connect(&env::var("DATABASE_URL").expect("No DATABASE_URL set."))
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Failed to connect to DB."))?;

    HttpServer::new(move || App::new().data(pool.clone()).service(hello))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[test]
    fn one_eq_one() {
        assert_eq!(1 + 1, 2);
    }

    #[actix_rt::test]
    async fn test_index_ok() {
        let mut app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
