use core::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

/// Alias for a `Result` with the error type `edn::Error`.
pub type Result<T> = result::Result<T, Error>;

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            msg.to_string(),
        ))
    }
}
