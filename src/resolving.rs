use resolve::{DnsResolver, DnsConfig, default_config};
use self::super::{CryptoAddress, Error};
use std::iter::FromIterator;
use std::time::Duration;
use std::str::FromStr;
use resolve::record;


pub fn addresses(address: &str) -> Result<Vec<CryptoAddress>, Error> {
    Ok(Result::from_iter(address_strings(address)?.into_iter().map(|s| CryptoAddress::from_str(&s)))?)
}

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
        .resolve_record::<record::Txt>(address)?
        .into_iter()
        .map(|r| r.data)
        .filter(|s| s.starts_with(b"oa1:"))
        .map(String::from_utf8))?)
}
