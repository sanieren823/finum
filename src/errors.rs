use crate::errors::FiNum::{Bcd, Bin, Bytes, Long};
use crate::finum::{FiBcd, FiBin, FiBytes, FiLong};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FiError {
    kind: FiErrorKind,
    msg: &'static str,
    number: FiNum,
}

#[derive(Debug, Clone)]
pub enum FiErrorKind {
    NumberTooLarge,
    NumberCannotBeNegative,
    ZeroIsAnInvalidInput,
}
#[derive(Debug, Clone)]
pub enum FiNum {
    Long(FiLong),
    Bin(FiBin),
    Bcd(FiBcd),
    Bytes(FiBytes),
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
    pub fn new(error_kind: FiErrorKind, message: &'static str, num: FiNum) -> FiError {
        FiError {
            kind: error_kind,
            msg: message,
            number: num,
        }
    }

    pub fn kind(self) -> FiErrorKind {
        self.kind
    }

    pub fn msg(self) -> &'static str {
        self.msg
    }

    pub fn num(self) -> FiNum {
        self.number
    }

    pub fn long(self) -> Option<FiLong> {
        match self.number {
            Long(num) => Some(num),
            _ => None,
        }
    }

    pub fn bin(self) -> Option<FiBin> {
        match self.number {
            Bin(num) => Some(num),
            _ => None,
        }
    }

    pub fn bcd(self) -> Option<FiBcd> {
        match self.number {
            Bcd(num) => Some(num),
            _ => None,
        }
    }

    pub fn bytes(self) -> Option<FiBytes> {
        match self.number {
            Bytes(num) => Some(num),
            _ => None,
        }
    }
}
