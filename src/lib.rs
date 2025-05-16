mod client;
mod error;
mod secret;
mod structs;

pub type Result<T> = std::result::Result<T, Error>;

pub use client::*;
pub use error::*;
pub use secret::*;
pub use structs::*;
