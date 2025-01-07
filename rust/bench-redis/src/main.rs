use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use redis::AsyncCommands;
use redis::Client;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_client = Arc::new(Client::open("redis://127.0.0.1:6379").unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(redis_client.clone()))
            .route("/", web::get().to(set_and_get))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn set_and_get(redis_client: Data<Arc<redis::Client>>) -> HttpResponse {
    let mut con = match redis_client.get_multiplexed_async_connection().await {
        Ok(conn) => conn,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let _: Result<(), redis::RedisError> = con.set("key", "BAR").await;

    let result: Result<String, redis::RedisError> = con.get("key").await;

    match result {
        Ok(val) => HttpResponse::Ok().body(val.to_uppercase()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
