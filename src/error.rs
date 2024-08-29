use hickory_resolver::error::ResolveError;
use std::error::Error as StdError;
use std::str::Utf8Error;
use std::io::Error as IoError;
use self::super::ParseError;
use std::convert::From;
use std::fmt;


/// Some error emerging from the library.
#[derive(Debug)]
pub enum Error {
    /// Error parsing an OA1 record.
    Oa1Parse(ParseError),
    /// Error with conversing with DNS server.
    Io(IoError),
    /// TXT record not UTF8 (ASCII).
    Utf8Parse(Utf8Error),
    /// TXT record not UTF8 (ASCII).
    Resolve(ResolveError),
    /// Non-FQDN address passed to `address*()`.
    AddressParse,
}

impl From<ParseError> for Error {
    fn from(pe: ParseError) -> Error {
        Error::Oa1Parse(pe)
    }
}

impl From<IoError> for Error {
    fn from(ioe: IoError) -> Error {
        Error::Io(ioe)
    }
}

impl From<Utf8Error> for Error {
    fn from(u8e: Utf8Error) -> Error {
        Error::Utf8Parse(u8e)
    }
}

impl From<ResolveError> for Error {
    fn from(re: ResolveError) -> Error {
        Error::Resolve(re)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Oa1Parse(ref pe) => pe.description(),
            Error::Io(ref ioe) => ioe.description(),
            Error::Utf8Parse(ref u8e) => u8e.description(),
            Error::Resolve(ref re) => re.description(),
            Error::AddressParse => "Specified address not valid OpenAlias",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Oa1Parse(ref pe) => Some(pe),
            Error::Io(ref ioe) => Some(ioe),
            Error::Utf8Parse(ref u8e) => Some(u8e),
            Error::Resolve(ref re) => Some(re),
            Error::AddressParse => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Oa1Parse(ref pe) => write!(f, "{}", pe),
            Error::Io(ref ioe) => write!(f, "{}", ioe),
            Error::Utf8Parse(ref u8e) => write!(f, "{}", u8e),
            Error::Resolve(ref re) => write!(f, "{}", re),
            Error::AddressParse => write!(f, "{}", self.description()),
        }
    }
}
