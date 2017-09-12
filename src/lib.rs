extern crate resolve;
extern crate crc;

mod error;
mod grammar;
mod resolving;
mod crypto_addr;

pub use self::error::Error;
pub use self::grammar::ParseError;
pub use self::crypto_addr::CryptoAddress;
pub use self::resolving::{address_strings, addresses};
