extern crate core;

pub mod api;
#[cfg(feature = "with_network")]
pub mod client;
pub mod error;
pub mod proto;
pub mod util;

// pub use self::client::CoinbaseSigner;
// pub use self::client::Nonce;
// pub use self::client::SignResult;
pub use self::error::*;
pub use self::proto::*;
pub use self::util::*;

#[cfg(feature = "with_network")]
pub use self::with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    // pub use super::api::spot::SpotApi;
    // pub use super::api::prime::PrimeApi;
}