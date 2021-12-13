use std::fmt;

#[derive(Debug, Clone)]
pub struct Error(String);

impl Error {
    pub fn new(message: String) -> Error {
        Error(message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
