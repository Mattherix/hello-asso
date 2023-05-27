//! `error` errors this crate can return

use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    ReqwestErr(reqwest::Error),
    AuthErr(AuthenticationError),
}

#[derive(Debug, Deserialize)]
pub struct AuthenticationError {
    pub error: String,
    pub error_description: String,
}
