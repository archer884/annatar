use config::resource::ResourceError;
use std::borrow::Cow;
use std::error;
use std::fmt;

pub struct Cause(pub Option<Box<error::Error>>);

impl Cause {
    pub fn none() -> Cause {
        Cause(None)
    }
}

impl<T: error::Error + 'static> From<T> for Cause {
    fn from(error: T) -> Cause {
        Cause(Some(Box::new(error)))
    }
}

impl fmt::Debug for Cause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "None"),
            Some(ref cause) => write!(f, "{}", cause),
        }
    }
}

#[derive(Debug)]
pub struct AppRunError {
    kind: AppRunErrorKind,
    description: Cow<'static, str>,
    cause: Cause,
}

#[derive(Debug)]
pub enum AppRunErrorKind {
    BadImage,
    IO,
    NotFound,
}

impl AppRunError {
    pub fn io<D, E>(desc: D, cause: E) -> AppRunError 
    where
        D: Into<Cow<'static, str>>,
        E: Into<Cause>,
    {
        AppRunError {
            kind: AppRunErrorKind::IO,
            description: desc.into(),
            cause: cause.into(),
        }
    }

    pub fn not_found<D, E>(desc: D, cause: E) -> AppRunError 
    where
        D: Into<Cow<'static, str>>,
        E: Into<Cause>,
    {
        AppRunError {
            kind: AppRunErrorKind::NotFound,
            description: desc.into(),
            cause: cause.into(),
        }
    }

    pub fn bad_image<E: Into<Cause>>(cause: E) -> AppRunError {
        AppRunError {
            kind: AppRunErrorKind::BadImage,
            description: Cow::from("We were unable to interpret this image"),
            cause: cause.into(),
        }
    }
}

impl fmt::Display for AppRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.description)
    }
}

impl error::Error for AppRunError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.cause.0 {
            Some(ref error) => Some(error.as_ref()),
            None => None,
        }
    }
}

impl From<ResourceError> for AppRunError {
    fn from(error: ResourceError) -> Self {
        AppRunError {
            kind: AppRunErrorKind::NotFound,
            description: Cow::from("Unable to retrieve the requested resource"),
            cause: error.into(),
        }
    }
}
