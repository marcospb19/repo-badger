use actix_web::http::HeaderValue;
use anyhow::Result;
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client,
};

pub fn build_client() -> Result<Client> {
    let mut headers = HeaderMap::new();

    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}
