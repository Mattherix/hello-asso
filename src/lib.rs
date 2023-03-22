use std::{collections::HashMap, time::{SystemTime, Duration}};

use derivative::Derivative;
use reqwest::header;
use serde::Deserialize;

const URL: &str = "https://api.helloasso.com/v5";
const OAUTH2_TOKEN_URL: &str = "https://api.helloasso.com/oauth2/token";
const OAUTH2_REFRESH_TOKEN_URL: &str = OAUTH2_TOKEN_URL;

#[derive(Clone, Derivative)]
#[derivative(Debug, PartialEq)]
struct HelloAsso {
    pub client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
    token_outdated_after: SystemTime,
    #[derivative(PartialEq = "ignore")]
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct RefreshToken {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: u64,
}

impl HelloAsso {
    fn builder(client_id: String, client_secret: String) -> HelloAssoBuilder {
        HelloAssoBuilder {
            client_id: client_id,
            client_secret: client_secret,
            access_token: None,
            refresh_token: None,
            token_type: None,
            expires_in: None,
            created_at: None,
            token_outdated_after: None,
            client: None,
        }
    }

    async fn refresh_token(&mut self) -> Result<&mut Self, reqwest::Error> {
        // Prepare request body
        let mut tokens = HashMap::new();
        tokens.insert("client_id", self.client_id.clone());
        tokens.insert("refresh_token", self.refresh_token.clone());
        tokens.insert("grant_type", "refresh_token".to_string());

        // Get access and refresh token
        let answer_client = reqwest::Client::new();
        let token = answer_client
            .post(OAUTH2_REFRESH_TOKEN_URL)
            .form(&tokens)
            .send()
            .await?
            .json::<RefreshToken>()
            .await?;

        // Fill data
        self.access_token = token.access_token;
        self.refresh_token = token.refresh_token;
        self.token_outdated_after = SystemTime::now()
             + Duration::from_secs(token.expires_in);

        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
struct HelloAssoBuilder {
    pub client_id: String,
    client_secret: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<u32>,
    created_at: Option<SystemTime>,
    token_outdated_after: Option<SystemTime>,
    #[serde(skip)]
    client: Option<reqwest::Client>,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: u64
}

impl HelloAssoBuilder {
    async fn get_token(&mut self) -> Result<&mut Self, reqwest::Error> {
        // Prepare request body
        let mut tokens = HashMap::new();
        tokens.insert("client_id", self.client_id.clone());
        tokens.insert(
            "client_secret",
            self.client_secret.clone(),
        );
        tokens.insert("grant_type", "client_credentials".to_string());

        // Get access and refresh token
        let answer_client = reqwest::Client::new();
        let token = answer_client
            .post(OAUTH2_TOKEN_URL)
            .form(&tokens)
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        // Fill data
        self.access_token = Some(token.access_token);
        self.refresh_token = Some(token.refresh_token);
        self.token_type = Some(token.token_type);
        self.token_outdated_after = Some(
            SystemTime::now() + Duration::from_secs(
                token.expires_in
            )
        );

        Ok(self)
    }

    fn config_client(&mut self) -> Result<&mut Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.access_token.clone().unwrap())
                .parse()
                .unwrap(),
        );
        self.client = Some(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        );

        Ok(self)
    }

    fn build(&mut self) -> HelloAsso {
        HelloAsso {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            access_token: self.access_token.clone().unwrap_or_default(),
            refresh_token: self.refresh_token.clone().unwrap_or_default(),
            token_outdated_after: self.token_outdated_after.unwrap_or(SystemTime::UNIX_EPOCH),
            client: self.client.clone().unwrap_or(reqwest::Client::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HelloAsso;

    #[tokio::test]
    async fn build_client() {
        HelloAsso::builder(
            "9a83d529ba764cf7ab04b2d377752d49".to_string(),
            "rca8GCvaE8pBo34gXvy7Rdb6k4bj2tUL".to_string(),
        )
        .get_token()
        .await
        .unwrap()
        .config_client()
        .unwrap()
        .build();
    }
}
