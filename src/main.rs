mod client;
mod error;
mod repo;

use actix_web::{get, web, App, HttpServer};

use error::{anyhow_to_bad_request, anyhow_to_internal_error};
use repo::fetch_repo;

#[get("/{username}/{repo}/badge.html")]
async fn badge(
    request: web::Path<(String, String)>,
) -> actix_web::Result<String> {
    let (username, repo) = request.into_inner();
    dbg!(&username, &repo);
    let client = client::build_client().map_err(anyhow_to_internal_error)?;
    let repo = fetch_repo(&client, &username, &repo)
        .await
        .map_err(anyhow_to_bad_request)?;
    Ok(format!(
        "TODO {} {} {}",
        repo.name, repo.username, repo.description
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(badge))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
