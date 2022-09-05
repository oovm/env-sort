use std::{
    env::VarError,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type XResult<T = ()> = Result<T, XError>;

#[derive(Debug, Clone)]
pub struct XError {
    message: String,
}

impl Display for XError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for XError {}

impl From<VarError> for XError {
    fn from(error: VarError) -> Self {
        XError { message: error.to_string() }
    }
}

impl From<std::io::Error> for XError {
    fn from(error: std::io::Error) -> Self {
        XError { message: error.to_string() }
    }
}
