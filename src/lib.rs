use std::{time::SystemTime};

use derivative::Derivative;
use serde::Deserialize;

const URL: &str = "https://api.helloasso.com/v5";
const OAUTH2_TOKEN_URL: &str = "https://api.helloasso.com/oauth2/token";

#[derive(Clone, Derivative)]
#[derivative(Debug, PartialEq)]
struct HelloAsso {
    pub client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
    token_outdated_after: SystemTime,
    #[derivative(PartialEq="ignore")]
    client: reqwest::Client
}

impl HelloAsso {
    fn builder(client_id: String, client_secret: String) -> HelloAssoBuilder {
        HelloAssoBuilder {
            client_id: Some(client_id),
            client_secret: Some(client_secret),
            access_token: None,
            refresh_token: None,
            token_type: None,
            expires_in: None,
            created_at: None,
            token_outdated_after: None,
            client: None
        }
    }
}

#[derive(Debug, Deserialize)]
struct HelloAssoBuilder {
    pub client_id: Option<String>,
    client_secret: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<i32>,
    created_at: Option<SystemTime>,
    token_outdated_after: Option<SystemTime>,
    #[serde(skip)]
    client: Option<reqwest::Client>
}

#[cfg(test)]
mod tests {
    use crate::HelloAsso;

    #[tokio::test]
    async fn build_client() {
        HelloAsso::builder(
            "9a83d529ba764cf7ab04b2d377752d49".to_string(),
            "rca8GCvaE8pBo34gXvy7Rdb6k4bj2tUL".to_string()
        );
    }
}
