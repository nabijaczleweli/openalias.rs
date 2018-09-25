//! Look up and parse [OpenAlias](https://openalias.org) data.
//!
//! # openalias.rs as а library
//!
//! This library can be used on a couple different levels (basic->high-level):
//!
//! 1. Parsing/validating OpenAlias "names" (e.g. "donate@getmonero.org", "nabijaczleweli.xyz")
//! 2. Parsing/validating OpenAlias records (e.g. "oa1:btc recipient_address=1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H;
//! recipient_name=Monero Development; tx_description=Donation to Monero Core Team;")
//! 3. Looking up OpenAliases with the DNS (e.g. "donate.nabijaczleweli.xyz" -> `CryptoAddress`)
//!
//! In that order, examples:
//!
//! ```
//! // Normally a user would type this in.
//! let address = "donate@getmonero.org";
//!
//! if let Some(fqdn) = openalias::alias_to_fqdn(address) {
//!     println!("{} maps to {}", address, fqdn);
//! #   assert_eq!(fqdn, "donate.getmonero.org.");
//! } else {
//!     // Address is not an OpenAlias
//! #   assert!(false);
//! }
//! ```
//!
//! Consult the [`alias_to_fqdn()`](fn.alias_to_fqdn.html) documentation for more information and examples.
//!
//! ```
//! # use std::collections::BTreeMap;
//! let record = "oa1:btc recipient_address=1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H; \
//!                       recipient_name=Monero Development; \
//!                       tx_description=Donation to Monero Core Team;";
//!
//! match record.parse::<openalias::CryptoAddress>() {
//!     Ok(ca) => {
//!         println!("{} address: {}", ca.cryptocurrency.to_uppercase(), ca.address);
//!         // Probably also handle more fields
//! #       assert_eq!(ca, openalias::CryptoAddress {
//! #           cryptocurrency: "btc".to_string(),
//! #           address: "1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H".to_string(),
//! #           recipient_name: Some("Monero Development".to_string()),
//! #           tx_description: Some("Donation to Monero Core Team".to_string()),
//! #           tx_amount: None,
//! #           tx_payment_id: None,
//! #           address_signature: None,
//! #           checksum: None,
//! #           additional_values: BTreeMap::new(),
//! #       });
//!     }
//!     Err(err) => {
//!         // The record is not an OpenAlias record,
//!         //   see err variable so as to the position of failure.
//! #       assert!(false);
//!     }
//! }
//! ```
//!
//! Consult the [`CryptoAddress`](struct.CryptoAddress.html) documentation for more information and examples.
//!
//! ```
//! # use std::collections::BTreeMap;
//! // Normally a user would type this in.
//! let alias = "donate.nabijaczleweli.xyz";
//!
//! match openalias::address_strings(alias) {
//!     Ok(cas) => {
//!         println!("{} addresses", cas.len());
//!         // cas contains "oa1:"-prefixed records
//! #       assert_eq!(cas, vec!["oa1:btc recipient_address=1CgLs6CxXMAY4Pj4edQq5vyaFoP9NdqVKH; recipient_name=nabijaczleweli; \
//! #                                      tx_description=Donation to nabijaczleweli;"]);
//!     }
//!     Err(err) => {
//!         // alias isn't an OpenAlias, or there was an error talking with a DNS server
//! #       assert!(false);
//!     }
//! }
//!
//! match openalias::addresses(alias) {
//!     Ok(cas) => {
//!         println!("{} addresses", cas.len());
//!         // cas contains CryptoAddresses
//! #       assert_eq!(cas, vec![openalias::CryptoAddress {
//! #           cryptocurrency: "btc".to_string(),
//! #           address: "1CgLs6CxXMAY4Pj4edQq5vyaFoP9NdqVKH".to_string(),
//! #           recipient_name: Some("nabijaczleweli".to_string()),
//! #           tx_description: Some("Donation to nabijaczleweli".to_string()),
//! #           tx_amount: None,
//! #           tx_payment_id: None,
//! #           address_signature: None,
//! #           checksum: None,
//! #           additional_values: BTreeMap::new(),
//! #       }]);
//!     }
//!     Err(err) => {
//!         // alias isn't an OpenAlias,
//!         //   or there was an error talking with a DNS server,
//!         //   or an "oa1:"-prefixed record isn't an OpenAlias record.
//! #       assert!(false);
//!     }
//! }
//! ```
//!
//! Consult the [`address_strings()`](fn.address_strings.html) and [`addresses()`](fn.addresses.html)
//! documentation for more information and examples.
//!
//! # openalias.rs as аn executable
//!
//! This is just a very short synopsis of
//! [the manpage](https://rawcdn.githack.com/nabijaczleweli/openalias.rs/man/openalias.1.html),
//! so consult that for more data.
//!
//! ## OPTIONS
//!
//! | Option                   | Description                                           |
//! |--------------------------|-------------------------------------------------------|
//! | &lt;OPEN_ALIAS&gt;...    | FQDN or email-style aliases to look up addresses for. |
//! | --verbose                | Print more data about what's happenning to stderr.    |
//! | --raw                    | Print just the record text.                           |
//! | --currency=[CURRENCY]... | Limit results to specified currencies.                |
//!
//! ## EXAMPLES
//!
//! `openalias nabijaczleweli.xyz donate.getmonero.org`
//!
//! ```text
//! Addresses of nabijaczleweli.xyz:
//!   btc:
//!     nabijaczleweli
//!     1CgLs6CxXMAY4Pj4edQq5vyaFoP9NdqVKH
//!
//! Addresses of donate.getmonero.org:
//!   xmr:
//!     Monero Development
//!     44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft3
//!       XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A
//!     Donation to Monero Core Team
//!   btc:
//!     Monero Development
//!     1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H
//!     Donation to Monero Core Team
//! ```
//!
//! `openalias -rv nabijaczleweli.xyz donate@getmonero.org`
//!
//! ```text
//! Looking up nabijaczleweli.xyz...
//! Addresses for nabijaczleweli.xyz:
//!   oa1:btc recipient_address=1CgLs6CxXMAY4Pj4edQq5vyaFoP9NdqVKH; recipient_name=nabijaczleweli;
//!
//! Looking up donate@getmonero.org...
//! Addresses for donate@getmonero.org:
//!   oa1:xmr recipient_address=44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft3
//!                               XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A;
//!           recipient_name=Monero Development; tx_description=Donation to Monero Core Team;
//!   oa1:btc recipient_address=1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H; recipient_name=Monero Development;
//!           tx_description=Donation to Monero Core Team;
//! ```
//!
//! `openalias -cxmr -c doge nabijaczleweli.xyz donate.getmonero.org`
//!
//! ```text
//! No xmr, nor doge addresses found for nabijaczleweli.xyz.
//!
//! Addresses of donate.getmonero.org:
//!   xmr:
//!     Monero Development
//!     44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft3
//!       XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A
//!     Donation to Monero Core Team
//! ```


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
