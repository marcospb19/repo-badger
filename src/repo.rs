use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::json::JsonObjectExt;

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
    /// Set to true if this repository is archived
    pub is_archived: bool,
}

impl Repository {
    /// Attempts to deserialize a Repository from a JSON
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
            is_archived: json.get_bool("archived")?,
        }
        .into()
    }
}

async fn make_get_request(client: &Client, url: String) -> Result<JsonValue> {
    let response = client.get(url).send().await?;

    // Check if our request failed
    response.error_for_status_ref()?;

    let response = response.text().await?;

    Ok(serde_json::from_str(&response)?)
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

#[cfg(test)]
mod tests {
    use super::build_repos_url;
    use super::Repository;

    const RESPONSE: &str = r###"
    {
        "id": 383324648,
        "node_id": "MDEwOlJlcG9zaXRvcnkzODMzMjQ2NDg=",
        "name": "bustd",
        "full_name": "vrmiguel/bustd",
        "private": false,
        "owner": {
          "login": "vrmiguel",
          "id": 36349314,
          "node_id": "MDQ6VXNlcjM2MzQ5MzE0",
          "avatar_url": "https://avatars.githubusercontent.com/u/36349314?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/vrmiguel",
          "html_url": "https://github.com/vrmiguel",
          "followers_url": "https://api.github.com/users/vrmiguel/followers",
          "following_url": "https://api.github.com/users/vrmiguel/following{/other_user}",
          "gists_url": "https://api.github.com/users/vrmiguel/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/vrmiguel/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/vrmiguel/subscriptions",
          "organizations_url": "https://api.github.com/users/vrmiguel/orgs",
          "repos_url": "https://api.github.com/users/vrmiguel/repos",
          "events_url": "https://api.github.com/users/vrmiguel/events{/privacy}",
          "received_events_url": "https://api.github.com/users/vrmiguel/received_events",
          "type": "User",
          "site_admin": false
        },
        "html_url": "https://github.com/vrmiguel/bustd",
        "description": "Process killer daemon for out-of-memory scenarios",
        "fork": false,
        "url": "https://api.github.com/repos/vrmiguel/bustd",
        "forks_url": "https://api.github.com/repos/vrmiguel/bustd/forks",
        "keys_url": "https://api.github.com/repos/vrmiguel/bustd/keys{/key_id}",
        "collaborators_url": "https://api.github.com/repos/vrmiguel/bustd/collaborators{/collaborator}",
        "teams_url": "https://api.github.com/repos/vrmiguel/bustd/teams",
        "hooks_url": "https://api.github.com/repos/vrmiguel/bustd/hooks",
        "issue_events_url": "https://api.github.com/repos/vrmiguel/bustd/issues/events{/number}",
        "events_url": "https://api.github.com/repos/vrmiguel/bustd/events",
        "assignees_url": "https://api.github.com/repos/vrmiguel/bustd/assignees{/user}",
        "branches_url": "https://api.github.com/repos/vrmiguel/bustd/branches{/branch}",
        "tags_url": "https://api.github.com/repos/vrmiguel/bustd/tags",
        "blobs_url": "https://api.github.com/repos/vrmiguel/bustd/git/blobs{/sha}",
        "git_tags_url": "https://api.github.com/repos/vrmiguel/bustd/git/tags{/sha}",
        "git_refs_url": "https://api.github.com/repos/vrmiguel/bustd/git/refs{/sha}",
        "trees_url": "https://api.github.com/repos/vrmiguel/bustd/git/trees{/sha}",
        "statuses_url": "https://api.github.com/repos/vrmiguel/bustd/statuses/{sha}",
        "languages_url": "https://api.github.com/repos/vrmiguel/bustd/languages",
        "stargazers_url": "https://api.github.com/repos/vrmiguel/bustd/stargazers",
        "contributors_url": "https://api.github.com/repos/vrmiguel/bustd/contributors",
        "subscribers_url": "https://api.github.com/repos/vrmiguel/bustd/subscribers",
        "subscription_url": "https://api.github.com/repos/vrmiguel/bustd/subscription",
        "commits_url": "https://api.github.com/repos/vrmiguel/bustd/commits{/sha}",
        "git_commits_url": "https://api.github.com/repos/vrmiguel/bustd/git/commits{/sha}",
        "comments_url": "https://api.github.com/repos/vrmiguel/bustd/comments{/number}",
        "issue_comment_url": "https://api.github.com/repos/vrmiguel/bustd/issues/comments{/number}",
        "contents_url": "https://api.github.com/repos/vrmiguel/bustd/contents/{+path}",
        "compare_url": "https://api.github.com/repos/vrmiguel/bustd/compare/{base}...{head}",
        "merges_url": "https://api.github.com/repos/vrmiguel/bustd/merges",
        "archive_url": "https://api.github.com/repos/vrmiguel/bustd/{archive_format}{/ref}",
        "downloads_url": "https://api.github.com/repos/vrmiguel/bustd/downloads",
        "issues_url": "https://api.github.com/repos/vrmiguel/bustd/issues{/number}",
        "pulls_url": "https://api.github.com/repos/vrmiguel/bustd/pulls{/number}",
        "milestones_url": "https://api.github.com/repos/vrmiguel/bustd/milestones{/number}",
        "notifications_url": "https://api.github.com/repos/vrmiguel/bustd/notifications{?since,all,participating}",
        "labels_url": "https://api.github.com/repos/vrmiguel/bustd/labels{/name}",
        "releases_url": "https://api.github.com/repos/vrmiguel/bustd/releases{/id}",
        "deployments_url": "https://api.github.com/repos/vrmiguel/bustd/deployments",
        "created_at": "2021-07-06T03:11:32Z",
        "updated_at": "2021-11-17T14:00:15Z",
        "pushed_at": "2021-11-14T04:58:43Z",
        "git_url": "git://github.com/vrmiguel/bustd.git",
        "ssh_url": "git@github.com:vrmiguel/bustd.git",
        "clone_url": "https://github.com/vrmiguel/bustd.git",
        "svn_url": "https://github.com/vrmiguel/bustd",
        "homepage": "",
        "size": 119,
        "stargazers_count": 135,
        "watchers_count": 135,
        "language": "Rust",
        "has_issues": true,
        "has_projects": true,
        "has_downloads": true,
        "has_wiki": true,
        "has_pages": false,
        "forks_count": 2,
        "mirror_url": null,
        "archived": false,
        "disabled": false,
        "open_issues_count": 1,
        "license": {
          "key": "mit",
          "name": "MIT License",
          "spdx_id": "MIT",
          "url": "https://api.github.com/licenses/mit",
          "node_id": "MDc6TGljZW5zZTEz"
        },
        "allow_forking": true,
        "is_template": false,
        "topics": [
          "daemon",
          "hackathon",
          "linux",
          "oomkiller",
          "out-of-memory",
          "rust"
        ],
        "visibility": "public",
        "forks": 2,
        "open_issues": 1,
        "watchers": 135,
        "default_branch": "master",
        "temp_clone_token": null,
        "network_count": 2,
        "subscribers_count": 3
      }
      
    "###;

    #[test]
    fn response_deserializes_into_repository() {
        let value: serde_json::Value = serde_json::from_str(&RESPONSE).unwrap();

        let repo = Repository::from_json(value).unwrap();

        assert_eq!(repo.username, "vrmiguel");
        assert_eq!(repo.name, "bustd");
        assert_eq!(repo.stars, 135);
        assert_eq!(repo.forks, 2);
        assert_eq!(repo.language.as_deref(), Some("Rust"));
        assert_eq!(repo.is_archived, false);
    }

    #[test]
    fn builds_repository_url_correctly() {
        assert_eq!(
            "https://api.github.com/repos/marcospb19/dotao",
            build_repos_url("marcospb19", "dotao")
        );

        assert_eq!("https://api.github.com/repos//", build_repos_url("", ""));
    }
}
