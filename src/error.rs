use std::error::Error as StdError;
use std::string::FromUtf8Error;
use std::io::Error as IoError;
use self::super::ParseError;
use std::convert::From;
use std::fmt;


#[derive(Debug)]
pub enum Error {
    Oa1Parse(ParseError),
    Io(IoError),
    Utf8Parse(FromUtf8Error),
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

impl From<FromUtf8Error> for Error {
    fn from(u8e: FromUtf8Error) -> Error {
        Error::Utf8Parse(u8e)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Oa1Parse(ref pe) => pe.description(),
            Error::Io(ref ioe) => ioe.description(),
            Error::Utf8Parse(ref u8e) => u8e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Oa1Parse(ref pe) => Some(pe),
            Error::Io(ref ioe) => Some(ioe),
            Error::Utf8Parse(ref u8e) => Some(u8e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Oa1Parse(ref pe) => write!(f, "{}", pe),
            Error::Io(ref ioe) => write!(f, "{}", ioe),
            Error::Utf8Parse(ref u8e) => write!(f, "{}", u8e),
        }
    }
}
