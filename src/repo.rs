use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

trait JsonValueExt {
    fn get_owner(&self) -> Option<String>;
    fn get_str(&self, _: &str) -> Option<String>;
    fn get_u64(&self, _: &str) -> Option<u64>;
}

impl JsonValueExt for JsonValue {
    fn get_owner(&self) -> Option<String> {
        match self {
            JsonValue::Object(obj) => {
                // `owner` is a map itself, which contains a `login` field
                let owner = obj.get("owner")?;
                owner.get("login")?.as_str().map(ToOwned::to_owned)
            }
            _ => None,
        }
    }

    fn get_str(&self, name: &str) -> Option<String> {
        dbg!(name);
        match self {
            JsonValue::Object(obj) => obj[name].as_str().map(ToOwned::to_owned),
            _ => None,
        }
    }

    fn get_u64(&self, name: &str) -> Option<u64> {
        dbg!(name);
        match self {
            JsonValue::Object(obj) => obj[name].as_u64(),
            _ => None,
        }
    }
}

async fn make_get_request(client: &Client, url: String) -> Result<JsonValue> {
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        // Something failed :/
        response.error_for_status_ref()?;
    }
    let response = response.text().await?;
    // dbg!(&response);
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
    /// The language this project is written in
    ///
    /// Optional since the GitHub API may return `null` if the repository
    /// is Markdown only
    pub language: Option<String>,
    /// The amount of stars this project has
    pub stars: u64,
    /// The amount of forks this project has
    pub forks: u64,
}

impl Repository {
    pub fn from_json(json: JsonValue) -> Option<Self> {
        // The JsonValue must be JsonValue::Object
        json.is_object().then(|| ())?;

        Self {
            username: json.get_owner()?,
            name: json.get_str("name")?,
            description: json.get_str("description")?,
            language: json.get_str("language"),
            stars: json.get_u64("stargazers_count")?,
            forks: json.get_u64("forks")?,
        }
        .into()
    }
}

/// Fetches a repository from GitHub given its owner/org. and name
///
/// Docs: https://docs.github.com/en/rest/reference/repos#get-a-repository
pub async fn fetch_repo(client: &Client, username: &str, repo: &str) -> Result<Repository> {
    let url = build_repos_url(username, repo);
    // dbg!(username, repo);
    let response = make_get_request(client, url).await?;
    let repository =
        Repository::from_json(response).with_context(|| "Failed to deserialize repository data")?;
    Ok(repository)
}

fn build_repos_url(username: &str, repository: &str) -> String {
    format!("https://api.github.com/repos/{}/{}", username, repository)
}
