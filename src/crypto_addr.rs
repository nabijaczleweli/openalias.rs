use self::super::{grammar, ParseError};
use crc::crc32::{self, Hasher32};
use std::collections::BTreeMap;
use std::{str, fmt, mem};
use std::io::Write;


/// [OpenAlias](https://openalias.org)-parsed cryptocurrency address.
///
/// `Display`ing an address with a checksum will *not* print out the same sum, but will re-hash the output string
/// (since the output can, while functionally equivalent, be different).
///
/// # Examples
///
/// Parse a simple example entry:
///
/// ```
/// # use openalias::CryptoAddress;
/// # use std::collections::BTreeMap;
/// static MONERO_DONATE_RCRD: &str =
///    "oa1:xmr \
///     recipient_address=46BeWrHpwXmHDpDEUmZBWZfoQpdc6HaERCNmx1pEYL2rAcu\
///                       wufPN9rXHHtyUA4QVy66qeFQkn6sfK8aHYjA3jk3o1Bv16em; \
///     recipient_name=Monero Development;";
/// assert_eq!(MONERO_DONATE_RCRD.parse::<CryptoAddress>().unwrap(),
///         CryptoAddress {
///             cryptocurrency: "xmr".to_string(),
///             address: "46BeWrHpwXmHDpDEUmZBWZfoQpdc6HaERCNmx1pEYL2rAcu\
///                       wufPN9rXHHtyUA4QVy66qeFQkn6sfK8aHYjA3jk3o1Bv16em".to_string(),
///
///             recipient_name: Some("Monero Development".to_string()),
///             tx_description: None,
///             tx_amount: None,
///             tx_payment_id: None,
///             address_signature: None,
///             checksum: None,
///
///             additional_values: BTreeMap::new(),
///         });
/// ```
///
/// Parse a more complex record:
///
/// ```
/// # use openalias::CryptoAddress;
/// # use std::collections::BTreeMap;
/// static NAB_DONATE_RCRD: &str =
///     "oa1:btc recipient_address=1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS; \
///      recipient_name=\"nabijaczleweli; FOSS development\";\
///      tx_description=Donation for nabijaczleweli:\\ ; \
///      tx_amount=0.1;checksum=D851342C; kaschism=yass;";
/// assert_eq!(NAB_DONATE_RCRD.parse::<CryptoAddress>().unwrap(),
///         CryptoAddress {
///             cryptocurrency: "btc".to_string(),
///             address: "1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS".to_string(),
///
///             recipient_name: Some("nabijaczleweli; FOSS development".to_string()),
///             tx_description: Some("Donation for nabijaczleweli: ".to_string()),
///             tx_amount: Some("0.1".to_string()),
///             tx_payment_id: None,
///             address_signature: None,
///             checksum: Some((0xD851342C, true)),
///
///             additional_values: {
///                 let mut avs = BTreeMap::new();
///                 avs.insert("kaschism".to_string(), "yass".to_string());
///                 avs
///             },
///         });
/// ```
///
/// `Display` a record:
///
/// ```
/// # use openalias::CryptoAddress;
/// # use std::collections::BTreeMap;
/// let mut base_record = CryptoAddress {
///     cryptocurrency: "btc".to_string(),
///     address: "1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS".to_string(),
///
///     recipient_name: Some("nabijaczleweli; FOSS development".to_string()),
///     tx_description: Some("Donation for nabijaczleweli: ".to_string()),
///     tx_amount: Some("0.1".to_string()),
///     tx_payment_id: None,
///     address_signature: None,
///     checksum: Some((0xD851342C, true)),
///
///     additional_values: {
///         let mut avs = BTreeMap::new();
///         avs.insert("kaschism".to_string(), "yass".to_string());
///         avs
///     },
/// };
///
/// assert_eq!(&base_record.to_string(),
///            "oa1:btc recipient_address=1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS; \
///             recipient_name=\"nabijaczleweli; FOSS development\"; \
///             tx_description=Donation for nabijaczleweli:\\ ; tx_amount=0.1; \
///             kaschism=yass; checksum=5AAC58F4;");
///
/// base_record.checksum = None;
/// assert_eq!(&base_record.to_string(),
///            "oa1:btc recipient_address=1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS; \
///             recipient_name=\"nabijaczleweli; FOSS development\"; \
///             tx_description=Donation for nabijaczleweli:\\ ; tx_amount=0.1; \
///             kaschism=yass;");
///
/// base_record.recipient_name = None;
/// base_record.tx_description = None;
/// base_record.tx_amount = None;
/// base_record.additional_values.clear();
/// assert_eq!(&base_record.to_string(),
///            "oa1:btc recipient_address=1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS;");
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CryptoAddress {
    /// Specified cryptocurrency's name.
    ///
    /// Usually "btc" for Bitcoin, "mxr" for Monero, et caetera.
    ///
    /// Note, that:
    ///
    /// > OpenAlias does not maintain a repository of prefixes at this stage, but may do so in future.
    pub cryptocurrency: String,
    /// Recipient's specified cryptocurrency address. Required.
    ///
    /// Corresponds to `recipient_address` record key.
    pub address: String,

    /// Recipient's specified user-friendlier name.
    ///
    /// Corresponds to `recipient_name` record key.
    pub recipient_name: Option<String>,
    /// Description for the transaction(s) resulting from this record.
    ///
    /// Note, that:
    ///
    /// > Bear in mind that DNS is typically long-lived data and not always updated at request time, so this should only be
    /// used if it does not need to be updated constantly.
    ///
    /// Corresponds to `tx_description` record key.
    pub tx_description: Option<String>,
    /// Amount of the specified cryptocurrency for the transaction(s) resulting from this record.
    ///
    /// Exact numeric value/type is usecase-dependent. No restrictions are applied within the realm of the library.
    ///
    /// Corresponds to `tx_amount` record key.
    pub tx_amount: Option<String>,
    /// "Particular to Monero, but is standardised as other cryptocurrencies (CryptoNote-based cryptocurrencies in particular)
    /// may find it useful."
    ///
    /// > It is typically a hex string of 32 characters, but that is not enforced in the standard.
    ///
    /// Corresponds to `tx_payment_id` record key.
    pub tx_payment_id: Option<String>,
    /// "If you have a standardised way of signing messages based on the address private key, then this can be used to validate
    /// the FQDN."
    ///
    /// > The message that is signed should be the entire FQDN (eg. donate.getmonero.org) with nothing else.
    /// Validation would be to verify that the signature is valid for the FQDN as a message.
    ///
    /// Corresponds to `address_signature` record key.
    pub address_signature: Option<String>,
    /// CRC-32 of the record up to this key.
    ///
    /// Second value of the pair is whether the checksum verified correctly, provided for convenience.
    ///
    /// > Depending on your use-case, it may serve little or no purpose, although some may choose to include it for additional
    /// validation. In order to calculate or verify the checksum, take the entire record up until the checksum key-value pair
    /// (ie. excluding the checksum key-value pair). Strip any spaces from either side, and calculate the CRC-32 on that final
    /// record.
    ///
    /// Corresponds to `checksum` record key.
    pub checksum: Option<(u32, bool)>,

    /// Set of K-Vs not special-cased above.
    pub additional_values: BTreeMap<String, String>,
}

impl str::FromStr for CryptoAddress {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<CryptoAddress, ParseError> {
        let mut out = grammar::oa1(s)?;

        if let Some(ref mut checksum) = out.checksum.as_mut() {
            let before_checksum = s[0..s.find("checksum").unwrap()].trim();
            let mut dgst = crc32::Digest::new(crc32::IEEE);
            dgst.write(before_checksum.as_bytes());
            checksum.1 = dgst.sum32() == checksum.0;
        }

        Ok(out)
    }
}

impl fmt::Display for CryptoAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn escape_val(out: &mut Vec<u8>, val: &str) -> fmt::Result {
            if val.contains(';') {
                out.push(b'"');
                write!(out, "{}", val).map_err(|_| fmt::Error)?;
                out.push(b'"');
                out.push(b';');
            } else if val.starts_with(' ') || val.ends_with(' ') {
                for i in 0..val.find(|c| c != ' ').unwrap_or(0) {
                    out.push(b'\\');
                    out.push(b' ');
                    println!("before {}", i);
                }
                write!(out, "{}", val.trim()).map_err(|_| fmt::Error)?;
                for i in val.rfind(|c| c != ' ').unwrap_or_else(|| val.len() - 1)..val.len() - 1 {
                    out.push(b'\\');
                    out.push(b' ');
                    println!("after {}/{}", i, val.len());
                }
                out.push(b';');
            } else {
                write!(out, "{};", val).map_err(|_| fmt::Error)?;
            }
            Ok(())
        }


        let mut out = vec![];

        write!(out, "oa1:{} recipient_address=", self.cryptocurrency).map_err(|_| fmt::Error)?;
        escape_val(&mut out, &self.address)?;

        if let Some(recipient_name) = self.recipient_name.as_ref() {
            write!(out, " recipient_name=").map_err(|_| fmt::Error)?;
            escape_val(&mut out, recipient_name)?;
        }

        if let Some(tx_description) = self.tx_description.as_ref() {
            write!(out, " tx_description=").map_err(|_| fmt::Error)?;
            escape_val(&mut out, tx_description)?;
        }

        if let Some(tx_amount) = self.tx_amount.as_ref() {
            write!(out, " tx_amount=").map_err(|_| fmt::Error)?;
            escape_val(&mut out, tx_amount)?;
        }

        if let Some(tx_payment_id) = self.tx_payment_id.as_ref() {
            write!(out, " tx_payment_id=").map_err(|_| fmt::Error)?;
            escape_val(&mut out, tx_payment_id)?;
        }

        if let Some(address_signature) = self.address_signature.as_ref() {
            write!(out, " address_signature=").map_err(|_| fmt::Error)?;
            escape_val(&mut out, address_signature)?;
        }

        for (key, val) in &self.additional_values {
            write!(out, " {}=", key).map_err(|_| fmt::Error)?;
            escape_val(&mut out, val)?;
        }

        f.write_str(str::from_utf8(&out).map_err(|_| fmt::Error)?)?;

        if self.checksum.is_some() {
            let mut dgst = crc32::Digest::new(crc32::IEEE);
            dgst.write(&out);
            write!(f, " checksum={:01$X};", dgst.sum32(), mem::size_of::<u32>() * 2).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}
