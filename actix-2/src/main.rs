use actix_web::{
    App, HttpResponse, HttpServer,
    web::{delete, get, post, put},
};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(|| async { HttpResponse::Ok().body("home") }))
            .route(
                "/hello",
                get().to(|| async { HttpResponse::Ok().body("hello") }),
            )
            .route(
                "/world",
                get().to(|| async { HttpResponse::Ok().body("world") }),
            )
            .route("/", post().to(|| async { HttpResponse::Ok().body("post") }))
            .route("/", put().to(|| async { HttpResponse::Ok().body("put") }))
            .route(
                "/",
                delete().to(|| async { HttpResponse::Ok().body("delete") }),
            )
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap()
}
