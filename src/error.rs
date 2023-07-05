//! Errors this crate can return

use std::fmt::Display;

use thiserror::Error;

use serde::Deserialize;

/// Errors that may occur when using the [client](crate::HelloAsso)
///
/// It can ether be a [Reqwest Error](reqwest::Error) or an [Authentication Error](crate::AuthenticationError)
// TODO: Decode error
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("request failed")]
    ReqwestErr(#[from] reqwest::Error),
    #[error("authentification failed")]
    AuthErr(AuthenticationError),
}

/// Authentication Error that may occur when trying to access the api
///
/// `helloasso` will raise an [AuthenticationError](crate::AuthenticationError) {
///     error: "unauthorized_client",
///     error_description: "Invalid client_id '{client_id}'"
/// } even when it is your `client_secret` witch is wrong.
/// This behaviour is from the web api.
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

#[cfg(test)]
mod tests {
    use crate::Error;
    use std::error::Error as StdError;

    fn error_trait_implemented<T>()
    where
        T: StdError,
    {
    }

    #[test]
    pub fn error_trait() {
        error_trait_implemented::<Error>();
    }
}
