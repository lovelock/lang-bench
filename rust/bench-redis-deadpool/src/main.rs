use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use deadpool_redis::{Config, Pool, Runtime};
use redis::AsyncCommands;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_url("redis://127.0.0.1:6379");
    let pool = Arc::new(config.create_pool(Some(Runtime::Tokio1)).unwrap());

    HttpServer::new(move || {
        let pool = pool.clone();
        App::new()
            .app_data(Data::new(pool))
            .route("/", web::get().to(set_and_get))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn set_and_get(pool: Data<Arc<Pool>>) -> HttpResponse {
    let mut con = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let _: Result<(), redis::RedisError> = con.set("key", "foobar").await;

    let result: Result<String, redis::RedisError> = con.get("key").await;

    match result {
        Ok(val) => HttpResponse::Ok().body(val.to_uppercase()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
