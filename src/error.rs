//! `error` errors this crate can return

use std::fmt::Display;

use thiserror::Error;

use serde::Deserialize;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("request failed")]
    ReqwestErr(#[from] reqwest::Error),
    #[error("authentifaication failed")]
    AuthErr(AuthenticationError),
}

#[derive(Error, Debug, Deserialize)]
pub struct AuthenticationError {
    pub error: String,
    pub error_description: String,
}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.error_description)
    }
}
