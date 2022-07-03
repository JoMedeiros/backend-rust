use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/* Essa função é passada explicitamente para o builer do App como uma rota
 * no métdo "route" */
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

