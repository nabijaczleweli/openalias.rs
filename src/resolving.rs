use self::super::{CryptoAddress, Error, alias_to_fqdn};
use resolve::{DnsResolver, DnsConfig, default_config};
use std::iter::FromIterator;
use std::time::Duration;
use resolve::record;


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
///                     address: "44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft\
///                               3XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A".to_string(),
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
///                 "oa1:xmr recipient_address=44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft\
///                  3XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A; \
///                  recipient_name=Monero Development; \
///                  tx_description=Donation to Monero Core Team;".to_string()]);
/// ```
pub fn address_strings(address: &str) -> Result<Vec<String>, Error> {
    Ok(Result::from_iter(DnsResolver::new(default_config().unwrap_or_else(|_| {
            DnsConfig {
                name_servers: vec!["8.8.8.8:53".parse().unwrap(), "8.8.4.4:53".parse().unwrap()],
                search: vec![],
                n_dots: 0,
                timeout: Duration::from_secs(5),
                attempts: 5,
                rotate: true,
                use_inet6: false,
            }
        }))
        ?
        .resolve_record::<record::Txt>(&alias_to_fqdn(address).ok_or(Error::AddressParse)?)?
        .into_iter()
        .map(|r| r.data)
        .filter(|s| s.starts_with(b"oa1:"))
        .map(String::from_utf8))?)
}
