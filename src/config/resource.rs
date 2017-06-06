use reqwest;
use std::error;
use std::fmt;
use std::io;

pub struct Resource(String);

#[derive(Debug)]
pub struct ResourceError(Box<error::Error>);

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
    fn description(&self) -> &str {
        error::Error::description(&*self.0)
    }

    fn cause(&self) -> Option<&error::Error> {
        error::Error::cause(&*self.0)
    }
}

impl Resource {
    pub fn new<T: Into<String>>(path: T) -> Resource {
        Resource(path.into())
    }

    pub fn get(&self) -> Result<Vec<u8>, ResourceError> {
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

fn load_web_resource(s: &str) -> Result<Vec<u8>, ResourceError> {
    use std::io::Read;
    let mut response = reqwest::get(s)?;
    let mut buf = Vec::new();
    response.read_to_end(&mut buf)?;
    Ok(buf)
}

fn load_local_resource(s: &str) -> Result<Vec<u8>, ResourceError> {
    use std::fs::File;
    use std::io::{BufReader, Read};
    let mut file = BufReader::new(File::open(s)?);
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
