use std::string::FromUtf8Error;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("the error {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
}
