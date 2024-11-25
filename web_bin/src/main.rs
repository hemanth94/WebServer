use std::time::Duration;
// use std::time::Instant;
use chrono::Local;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    let x = format!("Got this at time : {:?}", Local::now());
    tokio::time::sleep(Duration::from_micros(10)).await; // <-- Ok. Worker thread will handle other requests here
                                                         // println!("Sending response!");

    HttpResponse::Ok().body(x)
}

async fn numbers(value: web::Path<u32>) -> impl Responder {
    if value.clone() % 10 == 0 {
        tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    }
    let x = format!("Sending this {} at time : {:?}", value, Local::now());
    println!("Sending response! as {}", x);

    HttpResponse::Ok().body(x)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn _my_handler() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/number/{value}", web::get().to(numbers))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
