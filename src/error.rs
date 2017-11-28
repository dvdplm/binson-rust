use std;
use std::io;
use std::fmt::{self, Display};

use serde::ser;
// use serde::{ser, de};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Eof,
    Syntax,
    ExpectedBoolean,
    IoError(io::Error),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::Eof => "unexpected end of input",
            Error::Syntax => "unexpected end of input",
            Error::ExpectedBoolean => "unexpected end of input",
            Error::IoError(ref err) => err.description(),
        }
    }
}