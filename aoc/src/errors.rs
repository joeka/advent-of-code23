use std::fmt;

#[derive(Debug)]
pub struct AOCError {
    message: String,
}

impl AOCError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for AOCError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
