use actix_web::http::HeaderValue;
use anyhow::Result;
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client,
};

#[allow(clippy::declare_interior_mutable_const)]
const API_REST_V3: HeaderValue = HeaderValue::from_static("application/vnd.github.v3+json");
#[allow(clippy::declare_interior_mutable_const)]
const USER_AGENT: HeaderValue = HeaderValue::from_static("repobadger");

pub fn build_client() -> Result<Client> {
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, API_REST_V3);

    Ok(Client::builder()
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?)
}
