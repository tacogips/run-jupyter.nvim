use std::string::FromUtf8Error;
use thiserror::Error;
use tree_sitter::LanguageError;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("utf8 error: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("language error {0}")]
    LanguageError(#[from] LanguageError),

    #[error("unsupprted kernel {0}")]
    UnsuppotedKernel(String),
}
