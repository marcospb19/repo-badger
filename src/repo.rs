use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

trait JsonValueExt {
    fn get_str(&self, _: &str) -> Option<String>;
}

impl JsonValueExt for JsonValue {
    fn get_str(&self, name: &str) -> Option<String> {
        match self {
            JsonValue::Object(obj) => obj[name].as_str().map(ToOwned::to_owned),
            _ => None,
        }
    }
}

async fn make_get_request(client: &Client, url: String) -> Result<JsonValue> {
    let response = client.get(url).send().await?;
    let response = response.text().await?;
    Ok(serde_json::from_str(&response)?)
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    /// The owner or organization this repository
    /// belongs to
    pub username: String,
    /// The name of this repository
    pub name: String,
    /// Repository description
    pub description: String,
}

impl Repository {
    pub fn from_json(json: JsonValue) -> Option<Self> {
        // The JsonValue must be JsonValue::Object
        json.is_object().then(|| ())?;

        Self {
            username: json.get_str("username")?,
            name: json.get_str("name")?,
            description: json.get_str("bio/description/idk")?,
        }
        .into()
    }
}

/// Fetches a repository from GitHub given its owner/org. and name
///
/// Docs: https://docs.github.com/en/rest/reference/repos#get-a-repository
pub async fn fetch_repo(client: &Client, username: &str, repo: &str) -> Result<Repository> {
    let url = build_repos_url(username, repo);
    let response = make_get_request(client, url).await?;
    let repository =
        Repository::from_json(response).with_context(|| "Failed to deserialize repository data")?;
    Ok(repository)
}

fn build_repos_url(username: &str, repository: &str) -> String {
    format!("https://api.github.com/repos/{}/{}", username, repository)
}
