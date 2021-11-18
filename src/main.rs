mod client;
mod error;
mod repo;

use actix_web::{get, web, App, HttpServer};

use error::{anyhow_to_bad_request, anyhow_to_internal_error};
use repo::fetch_repo;

#[get("/{username}/{repo}/badge.html")]
/// Looks up the data of this repository and builds the SVG of its badge
async fn badge(request: web::Path<(String, String)>) -> actix_web::Result<String> {
    let (username, repo) = request.into_inner();

    println!("Received request for {}/{}", username, repo);

    // Builds a reqwest::Client with the appropriate Accepted and User-Agent headers set
    let client = client::build_client().map_err(anyhow_to_internal_error)?;

    // Looks up this repository through the GitHub API
    let repo = fetch_repo(&client, &username, &repo)
        .await
        .map_err(anyhow_to_bad_request)?;

    // (TODO) Generates the SVG for this badge
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
