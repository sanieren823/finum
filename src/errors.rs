use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FiError {
    kind: FiErrorKind,
    msg: &'static str,
}

#[derive(Debug)]
pub enum FiErrorKind {
    NumberTooLarge,
    NumberCannotBeNegative,

}
impl fmt::Display for FiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}


impl Error for FiError {
    fn description(&self) -> &str {
        self.msg
    }
}

impl FiError {
    pub fn kind(self) -> FiErrorKind {
        self.kind
    }
    pub fn new(error_kind: FiErrorKind, message: &'static str) -> FiError {
        FiError{kind: error_kind, msg: message}
    }
}