//! Errors this crate can return

use std::fmt::Display;

use thiserror::Error;

use serde::Deserialize;

/// Errors that may occur when using the [client](crate::HelloAsso)
///
/// It can ether be a [Reqwest Error](reqwest::Error) or an [Authentication Error](crate::AuthenticationError)
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("request failed")]
    ReqwestErr(#[from] reqwest::Error),
    #[error("authentification failed")]
    AuthErr(AuthenticationError),
    #[error("your don't have the right permission")]
    PermErr(AuthorizationError),
    #[error("can't decode request")]
    DecodeErr(reqwest::Error),
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

/// Authorization Error that may occur when trying to access the api
#[derive(Error, Debug, Deserialize)]
pub struct AuthorizationError {
    pub message: String,
}

impl Display for AuthorizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "authorization error: {}", self.message)
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

/*
Note to myself:

// TODO: Add test for PermErr, StatusCode::FORBIDDEN
By implementing an endpoint and using a mocker (ie https://github.com/lipanski/mockito)
We need to implement an endpoint first because the token url can't return a 401 or a 403

Example code for AuthenticationError and AuthorizationError:

StatusCode::UNAUTHORIZED => {
    let error = response.json::<AuthorizationError>().await.map_err(|err| {
        error!("Can't decode authentication error");
        Error::DecodeErr(err)
    })?;

    error!("An authentication error as occur, wrong jwt");

    Err(Error::AuthErr(error))
}

StatusCode::FORBIDDEN => {
    let error = response.json::<AuthorizationError>().await.map_err(|err| {
        error!("Can't decode authentication error");
        Error::DecodeErr(err)
    })?;

    error!("Your JWT token hasn't the privileges or Roles for this action");

    Err(Error::PermErr(error))
}


 */
