use std::fmt::{Debug, Display};
use thiserror::Error;

use crate::Rule;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParserError(#[from] Box<pest::error::Error<Rule>>),

    #[error("failed to resolve referenced input")]
    InputResolveError(String),

    #[error("attempted to use dot-notation on non-object value")]
    InvalidPathError(String),

    #[error("failed to deserialize input")]
    DeserializationError(String),
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
