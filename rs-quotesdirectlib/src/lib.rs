//! # Quotes Direct API
//!
//! [*CQG's Quotes Direct API*](https://help.cqg.com/apihelp/#!Documents/quotesdirectfixfast.htm) provides
//! fast and reliable market data feeds using the industry-standard FIX formats.
//!
//! This library provides structures, functions and methods for:
//! - reading TCP and UDP packets
//! - generating outcoming FIX messages
//! - parsing incoming FAST messages
//!
pub mod fast;
pub mod fix;
pub mod sync;

#[cfg(feature = "tokio")]
pub mod packets;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    ///! Errors happened due to invalid TCP packet length field or UDP packet length.
    #[error("Invalid packet length: {0}")]
    InvalidPacketLength(u64),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
