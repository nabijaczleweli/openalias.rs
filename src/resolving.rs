use self::super::{CryptoAddress, Error, alias_to_fqdn};
use hickory_resolver::Resolver as DnsResolver;
use std::iter::FromIterator;
use std::time::Duration;
use std::str;


/// Ask a DNS server for addresses for the specified OpenAlias.
///
/// # Examples
///
/// ```
/// # use openalias::{CryptoAddress, addresses};
/// # use std::collections::BTreeMap;
/// # trait SortedForVec { fn sorted(self) -> Self; }
/// # impl<T: Ord> SortedForVec for Vec<T> { fn sorted(mut self) -> Self { self.sort(); self} }
/// assert_eq!(addresses("donate@getmonero.org").unwrap().sorted(),
///            vec![CryptoAddress {
///                     cryptocurrency: "btc".to_string(),
///                     address: "1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H".to_string(),
///
///                     recipient_name: Some("Monero Development".to_string()),
///                     tx_description: Some("Donation to Monero Core Team".to_string()),
///                     tx_amount: None,
///                     tx_payment_id: None,
///                     address_signature: None,
///                     checksum: None,
///
///                     additional_values: BTreeMap::new(),
///                 },
///                 CryptoAddress {
///                     cryptocurrency: "xmr".to_string(),
///                     address: "888tNkZrPN6JsEgekjMnABU4TBzc2Dt29EPAvkRxbANsAnj\
///                               yPbb3iQ1YBRk1UXcdRsiKc9dhwMVgN5S9cQUiyoogDavup3H".to_string(),
///
///                     recipient_name: Some("Monero Development".to_string()),
///                     tx_description: Some("Donation to Monero Core Team".to_string()),
///                     tx_amount: None,
///                     tx_payment_id: None,
///                     address_signature: None,
///                     checksum: None,
///
///                     additional_values: BTreeMap::new(),
///                 }]);
/// ```
pub fn addresses(address: &str) -> Result<Vec<CryptoAddress>, Error> {
    Ok(Result::from_iter(address_strings(address)?.into_iter().map(|s| s.parse()))?)
}

/// Ask a DNS server for "oa1:"-prefixed TXT records for the specified OpenAlias.
///
/// # Examples
///
/// ```
/// # use openalias::address_strings;
/// # trait SortedForVec { fn sorted(self) -> Self; }
/// # impl<T: Ord> SortedForVec for Vec<T> { fn sorted(mut self) -> Self { self.sort(); self} }
/// assert_eq!(address_strings("donate@getmonero.org").unwrap().sorted(),
///            vec!["oa1:btc recipient_address=1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H; \
///                  recipient_name=Monero Development; \
///                  tx_description=Donation to Monero Core Team;".to_string(),
///                 "oa1:xmr recipient_address=888tNkZrPN6JsEgekjMnABU4TBzc2Dt29EPAvkRxbANsAnj\
///                  yPbb3iQ1YBRk1UXcdRsiKc9dhwMVgN5S9cQUiyoogDavup3H; \
///                  recipient_name=Monero Development; \
///                  tx_description=Donation to Monero Core Team;".to_string()]);
/// ```
pub fn address_strings(address: &str) -> Result<Vec<String>, Error> {
    Result::from_iter(DnsResolver::from_system_conf()
        ?
        .txt_lookup(alias_to_fqdn(address).ok_or(Error::AddressParse)?)?
        .iter()
        .flat_map(|t| t.iter())
        .filter(|s| s.starts_with(b"oa1:"))
        .map(|s| str::from_utf8(s).map(str::to_string).map_err(Error::Utf8Parse)))
}
