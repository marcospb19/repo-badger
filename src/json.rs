pub use serde_json::Value as JsonValue;

pub trait JsonObjectExt {
    fn get(&self, _: &str) -> Option<&JsonValue>;
    fn get_owner(&self) -> Option<String>;
    fn get_str(&self, _: &str) -> Option<String>;
    fn get_u64(&self, _: &str) -> Option<u64>;
    fn get_bool(&self, _: &str) -> Option<bool>;
}

impl JsonObjectExt for JsonValue {
    fn get(&self, key: &str) -> Option<&JsonValue> {
        self.as_object()?.get(key)
    }

    fn get_owner(&self) -> Option<String> {
        let owner = self.get("owner")?;
        owner.get_str("login")
    }

    fn get_str(&self, name: &str) -> Option<String> {
        self.get(name)?.as_str().map(ToOwned::to_owned)
    }

    fn get_u64(&self, name: &str) -> Option<u64> {
        self.get(name)?.as_u64()
    }

    fn get_bool(&self, name: &str) -> Option<bool> {
        self.get(name)?.as_bool()
    }
}

#[cfg(test)]
mod tests {
    use super::JsonObjectExt;

    const RESPONSE: &str = r###"
    {
        "id": 383324648,
        "node_id": "MDEwOlJlcG9zaXRvcnkzODMzMjQ2NDg=",
        "name": "bustd",
        "full_name": "vrmiguel/bustd",
        "private": false,
        "some_boolean": true,
        "another_boolean": false,
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
        }
    }"###;

    #[test]
    fn get_methods() {
        let response: serde_json::Value = serde_json::from_str(&RESPONSE).unwrap();

        assert_eq!(response.get_str("name").as_deref(), Some("bustd"));
        assert_eq!(response.get_str("unknown-name").as_deref(), None);

        assert_eq!(response.get_owner().as_deref(), Some("vrmiguel"));

        assert_eq!(response.get_u64("id"), Some(383324648));
        assert_eq!(response.get_u64("unknown-id"), None);

        assert_eq!(response.get_bool("some_boolean"), Some(true));
        assert_eq!(response.get_bool("another_boolean"), Some(false));

        assert_eq!(response.get_bool("unknown_boolean"), None);
    }
}
