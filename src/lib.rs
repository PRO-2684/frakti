#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(dead_code))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "client-cyper")]
pub use cyper;

pub use self::error::Error;
pub use self::parse_mode::ParseMode;
#[cfg(feature = "trait-async")]
pub use self::trait_async::AsyncTelegramApi;

#[cfg(feature = "client-cyper")]
pub mod client_cyper;
mod error;
pub mod games;
pub mod gifts;
pub mod inline_mode;
pub mod input_file;
pub mod input_media;
#[cfg(feature = "client-cyper")]
mod json;
mod macros;
pub mod methods;
mod parse_mode;
pub mod passport;
pub mod payments;
pub mod response;
pub mod stickers;
#[cfg(test)]
mod test_json;
#[cfg(feature = "trait-async")]
mod trait_async;
pub mod types;
pub mod updates;

/// Default Bot API URL
pub const BASE_API_URL: &str = "https://api.telegram.org/bot";

#[deprecated(
    since = "0.39.0",
    note = "enable the client-cyper feature and use frakti::client_cyper::Bot instead"
)]
#[doc(hidden)]
pub struct AsyncApi;
