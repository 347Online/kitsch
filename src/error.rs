use std::fmt::{Display};

#[derive(Debug)]
pub struct ShellError {
    // kind: 
    message: String,
}

impl ShellError {
    pub fn new(message: &str) -> Self {
        ShellError {
            message: String::from(message)
        }
    }
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}