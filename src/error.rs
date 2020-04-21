use crate::config::resource::ResourceError;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Image(artano::Error),
    IO(io::Error),
    Resource(ResourceError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Image(e) => write!(f, "Bad image: {}", e),
            Error::Resource(e) => write!(f, "Unable to retreive the requested resource: {}", e),
            Error::IO(e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IO(e) => Some(e),
            Error::Resource(e) => Some(e),

            _ => None,
        }
    }
}

impl From<artano::Error> for Error {
    fn from(e: artano::Error) -> Self {
        Error::Image(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}
impl From<ResourceError> for Error {
    fn from(e: ResourceError) -> Self {
        Error::Resource(e)
    }
}
