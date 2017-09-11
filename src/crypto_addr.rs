use self::super::{grammar, ParseError};
use crc::crc32::{self, Hasher32};
use std::collections::BTreeMap;
use std::{str, fmt, mem};
use std::io::Write;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CryptoAddress {
    pub cryptocurrency: String,
    pub address: String,

    pub recipient_name: Option<String>,
    pub tx_description: Option<String>,
    pub tx_amount: Option<String>,
    pub tx_payment_id: Option<String>,
    pub address_signature: Option<String>,
    pub checksum: Option<(u32, bool)>,

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
        let mut out = vec![];

        write!(out, "oa1:{} recipient_address={};", self.cryptocurrency, self.address).map_err(|_| fmt::Error)?;

        if let Some(ref recipient_name) = self.recipient_name.as_ref() {
            write!(out, " recipient_name={};", recipient_name).map_err(|_| fmt::Error)?;
        }

        if let Some(ref tx_description) = self.tx_description.as_ref() {
            write!(out, " tx_description={};", tx_description).map_err(|_| fmt::Error)?;
        }

        if let Some(ref tx_amount) = self.tx_amount.as_ref() {
            write!(out, " tx_amount={};", tx_amount).map_err(|_| fmt::Error)?;
        }

        if let Some(ref tx_payment_id) = self.tx_payment_id.as_ref() {
            write!(out, " tx_payment_id={};", tx_payment_id).map_err(|_| fmt::Error)?;
        }

        if let Some(ref address_signature) = self.address_signature.as_ref() {
            write!(out, " address_signature={};", address_signature).map_err(|_| fmt::Error)?;
        }

        for (ref key, ref val) in &self.additional_values {
            write!(out, " {}={};", key, val).map_err(|_| fmt::Error)?;
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
