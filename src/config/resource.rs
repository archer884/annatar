use std::{error, fmt, io, result};

type Result<T> = result::Result<T, ResourceError>;

#[derive(Debug)]
pub struct ResourceError(Box<dyn error::Error + 'static>);

#[derive(Clone, Debug)]
pub struct Resource(String);

impl From<io::Error> for ResourceError {
    fn from(error: io::Error) -> Self {
        ResourceError(Box::new(error))
    }
}

impl From<reqwest::Error> for ResourceError {
    fn from(error: reqwest::Error) -> Self {
        ResourceError(Box::new(error))
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource error: {}", self.0)
    }
}

impl error::Error for ResourceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl Resource {
    pub fn new<T: Into<String>>(path: T) -> Resource {
        Resource(path.into())
    }

    pub fn get(&self) -> Result<Vec<u8>> {
        if self.is_http() {
            load_web_resource(&self.0)
        } else {
            load_local_resource(&self.0)
        }
    }

    fn is_http(&self) -> bool {
        self.0.starts_with("http://") || self.0.starts_with("https://")
    }
}

fn load_web_resource(s: &str) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    reqwest::blocking::get(s)?.copy_to(&mut buf)?;
    Ok(buf)
}

fn load_local_resource(s: &str) -> Result<Vec<u8>> {
    use std::fs;
    Ok(fs::read(s)?)
}
