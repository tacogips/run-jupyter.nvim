pub mod comment_extractor;
pub mod error;
pub mod python_parser;
pub mod rust_parser;
use jupyter_client::CellType;

pub type Result<T> = std::result::Result<T, error::ParserError>;

#[derive(Debug, PartialEq)]
pub struct CellSources {
    pub cell_sources: Vec<CellSource>,
}

impl CellSources {
    pub fn push(&mut self, c: CellSource) {
        self.cell_sources.push(c)
    }

    pub fn len(&self) -> usize {
        self.cell_sources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
}

trait CodeParser {
    fn parse(&mut self, code: &str) -> Result<Option<CellSources>>;
}
