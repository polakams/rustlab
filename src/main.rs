use actix_web::{App, get, HttpServer};

#[get("/greet")]
async fn hello_world() -> String {
    return "Hello, World!".to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_world)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}