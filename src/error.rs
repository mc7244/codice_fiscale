#[derive(Debug, Clone)]
pub struct CFError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

use std;
use std::fmt;

impl fmt::Display for CFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl std::error::Error for CFError {
    fn description(&self) -> &str {
        &self.message
    }
}