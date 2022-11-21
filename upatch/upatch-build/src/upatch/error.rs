use std::{io, fmt::{self, Debug}};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Compiler(String),
    Project(String),
    Build(String),
    Io(String),
    Mod(String),
    Diff(String),
    InvalidInput(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(format!("{}", err))
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

impl Error {
    pub fn description(&self) -> &String {
        match *&self {
            Error::Io(err) => err,
            Error::Compiler(err) => err,
            Error::Project(err) => err,
            Error::Build(err) => err,
            Error::Mod(err) => err,
            Error::Diff(err) => err,
            Error::InvalidInput(err) => err,
        }
    }

    pub fn code(&self) -> i32 {
        match *&self {
            Error::Io(_) => 1,
            Error::Compiler(_) => 2,
            Error::Project(_) => 3,
            Error::Build(_) => 4,
            Error::Mod(_) => 5,
            Error::Diff(_) => 6,
            Error::InvalidInput(_) => 7,
        }
    }
}