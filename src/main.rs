use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;
use tokio::time;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let var_name = async {
        loop {
            println!("Hello"); // Print "Hello" to the console
            time::sleep(Duration::from_secs(60)).await; // Wait for 60 seconds
        }
    };
    tokio::spawn(var_name);

    println!("Server starter!");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
