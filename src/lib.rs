//! # Hello Asso
//!
//! `helloasso` is a create used to interact with the [helloasso api](https://api.helloasso.com/v5/swagger/ui/index#/).
//! It is not affiliated to helloasso.
mod client;
mod error;

pub use crate::client::HelloAsso;
pub use crate::error::{AuthenticationError, AuthorizationError, Error};
