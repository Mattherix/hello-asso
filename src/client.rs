//! `client` the client and client builder

use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use derivative::Derivative;
#[cfg(feature = "log")]
use log::{error, info};
use reqwest::{header, StatusCode};
use serde::Deserialize;

use crate::{error::Error, AuthenticationError};

// const URL: &str = "https://api.helloasso.com/v5";
const OAUTH2_TOKEN_URL: &str = "https://api.helloasso.com/oauth2/token";
const OAUTH2_REFRESH_TOKEN_URL: &str = OAUTH2_TOKEN_URL;

#[derive(Clone, Derivative)]
#[derivative(Debug, PartialEq)]
pub struct HelloAsso {
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
    /* token_type: String, */
    expires_in: u64,
}

impl HelloAsso {
    /// Create a new client to interact with the api
    pub async fn new(client_id: String, client_secret: String) -> Result<Self, Error> {
        let client = HelloAsso::builder(client_id, client_secret)
            .get_token()
            .await?
            .config_client()?
            .build();

        #[cfg(feature = "log")]
        info!("New client created");

        Ok(client)
    }

    /// Create a client builder that can be configure
    ///
    /// The helloasso client can be created ether by calling the `new` method
    /// or by using the builder pattern for a higher flexibility.
    ///
    /// ```rust
    /// # use helloasso::{HelloAsso, Error};
    /// # use dotenv::dotenv;
    /// # use std::env;
    /// #
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), Error> {
    /// # dotenv();
    /// # let client_id = env::var("CLIENT_ID").unwrap();
    /// # let client_secret = env::var("CLIENT_SECRET").unwrap();
    /// #
    /// let client = HelloAsso::builder(client_id, client_secret)
    ///     .get_token()
    ///     .await?
    ///     .config_client()?
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder(client_id: String, client_secret: String) -> HelloAssoBuilder {
        HelloAssoBuilder {
            client_id,
            client_secret,
            access_token: None,
            refresh_token: None,
            token_type: None,
            token_outdated_after: None,
            client: None,
        }
    }

    /// Refresh the access_token of the client
    ///
    /// By default access token are only valid for 30 min,
    /// we can use this function to reset this timer
    pub async fn refresh_token(&mut self) -> Result<&mut Self, reqwest::Error> {
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
            .await
            .map_err(|err| {
                #[cfg(feature = "log")]
                error!("Can't fetch refresh token from the api");
                err
            })?
            .json::<RefreshToken>()
            .await
            .map_err(|err| {
                #[cfg(feature = "log")]
                error!("Can't deserialize refresh token response");
                err
            })?;

        // Fill data
        self.access_token = token.access_token;
        self.refresh_token = token.refresh_token;
        self.token_outdated_after = SystemTime::now() + Duration::from_secs(token.expires_in);

        #[cfg(feature = "log")]
        info!("Access token refreshed");
        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct HelloAssoBuilder {
    pub client_id: String,
    client_secret: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_type: Option<String>,
    token_outdated_after: Option<SystemTime>,
    #[serde(skip)]
    client: Option<reqwest::Client>,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: u64,
}

impl HelloAssoBuilder {
    /// Get the access token using the client id an secret
    pub async fn get_token(&mut self) -> Result<&mut Self, Error> {
        // Prepare request body
        let mut tokens = HashMap::new();
        tokens.insert("client_id", self.client_id.clone());
        tokens.insert("client_secret", self.client_secret.clone());
        tokens.insert("grant_type", "client_credentials".to_string());

        // Get access and refresh token
        let answer_client = reqwest::Client::new();
        let response = answer_client
            .post(OAUTH2_TOKEN_URL)
            .form(&tokens)
            .send()
            .await
            .map_err(|err| {
                #[cfg(feature = "log")]
                error!("Can't fetch access token");
                Error::ReqwestErr(err)
            })?;

        match response.status() {
            StatusCode::OK => {
                let token = response
                    .json::<AccessTokenResponse>()
                    .await
                    .map_err(|err| {
                        #[cfg(feture = "log")]
                        error!("Can't decode access token");
                        Error::DecodeErr(err)
                    })?;

                // Fill data
                self.access_token = Some(token.access_token);
                self.refresh_token = Some(token.refresh_token);
                self.token_type = Some(token.token_type);
                self.token_outdated_after =
                    Some(SystemTime::now() + Duration::from_secs(token.expires_in));

                info!("Access token fetched");

                Ok(self)
            }
            StatusCode::BAD_REQUEST => {
                let error = response
                    .json::<AuthenticationError>()
                    .await
                    .map_err(|err| {
                        #[cfg(feture = "log")]
                        error!("Can't decode authentication error");
                        Error::DecodeErr(err)
                    })?;

                #[cfg(feature = "log")]
                error!("An authentication error as occur, wrong client_id or credential");

                Err(Error::AuthErr(error))
            }
            status => {
                unimplemented!(
                    "Unknown status code while fetching the access_token, {}",
                    status
                )
            }
        }
    }

    /// Create a new client using a previously set access_token, see `get_token`
    pub fn config_client(&mut self) -> Result<&mut Self, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!(
                "Bearer {}",
                self.access_token
                    .clone()
                    .expect("Can't get the access_token, use get_token")
            )
            .parse()
            .expect("Can't parse formatted token into a HeaderName"),
        );
        self.client = Some(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .map_err(Error::ReqwestErr)?,
        );

        #[cfg(feature = "log")]
        info!("Client configured");
        Ok(self)
    }

    /// Build the client
    pub fn build(&mut self) -> HelloAsso {
        HelloAsso {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            access_token: self.access_token.clone().unwrap_or_default(),
            refresh_token: self.refresh_token.clone().unwrap_or_default(),
            token_outdated_after: self.token_outdated_after.unwrap_or(SystemTime::UNIX_EPOCH),
            client: self.client.clone().unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, HelloAsso};
    use dotenv::dotenv;
    #[cfg(feature = "log")]
    use log::{info, warn};
    use std::env;

    pub fn get_env_variables() -> (String, String) {
        if let Err(err) = dotenv() {
            #[cfg(feature = "log")]
            warn!("Can't load .env file, {}", err);
        } else {
            #[cfg(feature = "log")]
            info!(".env file loaded");
        }

        let client_id = env::var("CLIENT_ID").unwrap();
        let client_secret = env::var("CLIENT_SECRET").unwrap();

        (client_id, client_secret)
    }

    #[tokio::test]
    async fn new_client() {
        let (client_id, client_secret) = get_env_variables();

        HelloAsso::new(client_id, client_secret)
            .await
            .expect("Test failed");
    }

    #[tokio::test]
    async fn invalid_client_id() {
        let (_, client_secret) = get_env_variables();
        let client_id = "abc".to_string();

        let client = HelloAsso::new(client_id, client_secret).await;

        assert!(matches!(client, Err(Error::AuthErr(_))))
    }

    #[tokio::test]
    async fn invalid_client_secret() {
        let (client_id, _) = get_env_variables();
        let client_secret = "abc".to_string();

        let client = HelloAsso::new(client_id, client_secret).await;

        assert!(matches!(client, Err(Error::AuthErr(_))))
    }

    #[tokio::test]
    async fn refresh_token() {
        let (client_id, client_secret) = get_env_variables();

        let mut client = HelloAsso::new(client_id, client_secret)
            .await
            .expect("Can't create the client");

        client
            .refresh_token()
            .await
            .expect("Could not refresh token");
    }
}
