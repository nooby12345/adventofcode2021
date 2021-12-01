use std::fmt::{Display, Formatter};
use std::io::Error;
use crate::AOCError::GeneralError;

#[derive(Debug)]
pub enum AOCError {
    GeneralError(String),
}

impl From<std::io::Error> for AOCError {
    fn from(io_error: Error) -> Self {
        GeneralError(format!("{}", io_error))
    }
}

impl Display for AOCError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}