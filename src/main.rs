use actix_web::{get, App, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    str: String,
    num: i32,
    arr: Vec::<i32>,
}

#[get("/getjson")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(MyObj {
        str: "test2".to_string(),
        num: 100,
        arr: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    })

    // HttpResponse::Ok()
    // .content_type("application/json")
    // .body(r#"{"str":"test1","num":100,"arr":[1,2,3,4,5]}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
