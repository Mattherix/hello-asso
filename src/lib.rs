//! # Hello Asso
//! 
//! `helloasso` is a create used to interact with [helloasso api](https://api.helloasso.com/v5/swagger/ui/index#/).
//! It is not affiliated to helloasso.
mod client;
pub use crate::client::{Error, HelloAsso};
