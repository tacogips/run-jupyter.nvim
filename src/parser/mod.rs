pub mod comment_extractor;
pub mod error;
pub mod rust_parser;

pub use error::*;
use jupyter_client::CellType;
pub use rust_parser::*;

type Result<T> = std::result::Result<T, error::ParserError>;

pub enum ParsableKernel {
    Rust,
    Python3,
}

impl ParsableKernel {
    pub fn try_from_str(kernel: &str) -> Result<Self> {
        match kernel {
            "rust" => Ok(Self::Rust),
            "python3" => Ok(Self::Python3),
            other => Err(error::ParserError::UnsuppotedKernel(format!(
                "Not supported kernel :{other}"
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CellSources {
    pub cell_sources: Vec<CellSource>,
}

impl CellSources {
    pub fn push(&mut self, c: CellSource) {
        self.cell_sources.push(c)
    }

    pub fn as_one_line_code(&self) -> String {
        self.cell_sources
            .iter()
            .map(|cell| cell.as_one_line_code().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Default for CellSources {
    fn default() -> Self {
        Self {
            cell_sources: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CellSource {
    pub cell_type: CellType,
    pub codes: Vec<String>,
}

impl Default for CellSource {
    fn default() -> Self {
        Self {
            cell_type: CellType::Code,
            codes: vec![],
        }
    }
}

impl CellSource {
    pub fn new_code(codes: Vec<String>) -> Self {
        Self {
            cell_type: CellType::Code,
            codes,
        }
    }

    pub fn len(&self) -> usize {
        self.codes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, s: String) {
        self.codes.push(s)
    }

    pub fn as_one_line_code(&self) -> String {
        self.codes.join("\n")
    }
}

pub trait CodeParser {
    fn parse(&mut self, code: &str) -> Result<Option<CellSources>>;
}
