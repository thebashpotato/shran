use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Used to easily read and write github auth information
/// to disk in yaml format. Currently only supports a token
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GithubAuth {
    github_authentication: HashMap<String, String>,
}

impl GithubAuth {
    /// Build a serializable/deserializable Token structure
    pub fn new(token: &String) -> Self {
        let mut github_authentication = HashMap::new();
        let _ = github_authentication.insert(String::from("token"), token.to_owned());

        Self {
            github_authentication,
        }
    }

    pub fn extract_token(&self) -> String {
        if let Some(t) = self.github_authentication.get("token") {
            return t.to_owned();
        }
        String::from("")
    }
}
