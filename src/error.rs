use config::resource::ResourceError;
use std::borrow::Cow;
use std::error;
use std::fmt;

pub type Cause = Option<Box<::std::error::Error>>;

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
    pub fn io<D: Into<Cow<'static, str>>>(desc: D, cause: Cause) -> AppRunError {
        AppRunError {
            kind: AppRunErrorKind::IO,
            description: desc.into(),
            cause,
        }
    }
    
    pub fn not_found<D: Into<Cow<'static, str>>>(desc: D, cause: Cause) -> AppRunError {
        AppRunError {
            kind: AppRunErrorKind::NotFound,
            description: desc.into(),
            cause,
        }
    }

    pub fn bad_image(cause: Cause) -> AppRunError {
        AppRunError {
            kind: AppRunErrorKind::BadImage,
            description: Cow::from("We were unable to interpret this image"),
            cause,
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
        match self.cause {
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
            cause: Some(Box::new(error)),
        }
    }
}
