use std::env::VarError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type XResult<T = ()> = Result<T, XError>;

#[derive(Debug, Clone)]
pub enum XError {
    UnknownError,
    RuntimeError {
        message: String,
    },
}

impl Display for XError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for XError {

}


impl From<VarError> for XError {
    fn from(value: VarError) -> Self {
        XError::RuntimeError {
            message: value.to_string(),
        }
    }
}