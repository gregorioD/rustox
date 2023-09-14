use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    line: usize,
    message: String,
}

impl Error {
    pub fn new(line: usize, message: String) -> Self {
        Error {
            line: line,
            message: message,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR at {}: {})", self.line(), self.message)
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        &self.message[..]
    }
}
