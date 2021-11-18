use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{username}/{repo}/badge.html")]
async fn badge(web::Path((username, repo)): web::Path<(String, String)>) -> impl Responder {
    format!("card for {}/{}", username, repo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(badge))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
