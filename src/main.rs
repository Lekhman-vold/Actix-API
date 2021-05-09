use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder, Result};
use serde::Deserialize;
use std::cell::Cell;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Server started!");

    let data = AppState {
        count: Cell::new(0)
    };

    HttpServer::new(move || {
        App::new()
            .service(user)
            .service(hello)
            .service(echo)
            .service(get_user)
            .service(get_books)
            .service(get_request)
            .data(data.clone())
            .route("/data", web::to(show_count))
            .route("/data-add", web::to(add_one))
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey Rust!")
}

#[get("/get-request")]
async fn get_request(_req: HttpRequest) -> impl Responder {
    "test string".to_owned()
}


#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[derive(Deserialize)]
struct UserInfo {
    username: String
}

#[derive(Clone)]
struct AppState {
    count: Cell<usize>
}

#[get("/user")]
async fn user(info: web::Json<UserInfo>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[get("/users/{user_id}/{friend}")]
async fn get_user(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", info.friend, info.user_id))
}

#[get("/books/{book_id}/{book_name}")]
async fn get_books(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("book_name").unwrap().parse().unwrap();
    let book_id: i32 = req.match_info().query("book_id").parse().unwrap();

    Ok(format!("Book: {}, book ID {}!", name, book_id))
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}
