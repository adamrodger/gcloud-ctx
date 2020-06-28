use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    ConfigurationStoreNotFound,
    UnableToReadConfigurations,
    UnknownConfiguration,
    Io(io::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<glob::GlobError> for Error {
    fn from(_: glob::GlobError) -> Self {
        Error::UnableToReadConfigurations
    }
}
