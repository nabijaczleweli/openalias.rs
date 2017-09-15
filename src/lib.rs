extern crate resolve;
#[macro_use]
extern crate clap;
extern crate crc;

mod error;
mod grammar;
mod address;
mod options;
mod resolving;
mod crypto_addr;

pub use self::error::Error;
pub use self::options::Options;
pub use self::grammar::ParseError;
pub use self::address::alias_to_fqdn;
pub use self::crypto_addr::CryptoAddress;
pub use self::resolving::{address_strings, addresses};
