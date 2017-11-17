use config::resource::ResourceError;
use std::borrow::Cow;
use std::error;
use std::fmt;

pub type Cause = Box<error::Error>;

pub trait IntoCause: error::Error + Sized + 'static {
    fn into(self) -> Cause {
        Box::new(self)
    }
}

impl<T: error::Error + Sized + 'static> IntoCause for T { }

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    description: Cow<'static, str>,
    cause: Option<Cause>,
}

#[derive(Debug)]
pub enum ErrorKind {
    BadImage,
    IO,
    NotFound,
}

impl Error {
    pub fn io<D, E>(desc: D, error: E) -> Self
    where
        D: Into<Cow<'static, str>>,
        E: IntoCause,
    {
        Self {
            kind: ErrorKind::IO,
            description: desc.into(),
            cause: Some(error.into()),
        }
    }

    pub fn not_found<D, E>(desc: D, error: E) -> Self
    where
        D: Into<Cow<'static, str>>,
        E: IntoCause,
    {
        Self {
            kind: ErrorKind::NotFound,
            description: desc.into(),
            cause: Some(error.into()),
        }
    }

    pub fn bad_image<E: IntoCause>(error: E) -> Self {
        Self {
            kind: ErrorKind::BadImage,
            description: Cow::from("We were unable to interpret this image"),
            cause: Some(error.into()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.description)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        self.cause.as_ref().map(|cause| cause.as_ref())
    }
}

impl From<ResourceError> for Error {
    fn from(error: ResourceError) -> Self {
        Self {
            kind: ErrorKind::NotFound,
            description: Cow::from("Unable to retrieve the requested resource"),
            cause: Some(Box::new(error)),
        }
    }
}
