pub mod comment_extractor;
pub mod error;
pub mod python_parser;
pub mod rust_parser;
use jupyter_client::CellType;

pub type Result<T> = std::result::Result<T, error::ParserError>;

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
        self.cell_sources.is_empty()
    }
}

impl CellSources {
    fn default() -> Self {
        Self {
            cell_sources: vec![],
        }
    }
}

//pub enum{
//}

pub struct CellSource {
    pub codes: Vec<String>,
}

impl CellSource {
    fn default() -> Self {
        Self { codes: vec![] }
    }
}

impl CellSource {
    pub fn len(&self) -> usize {
        self.codes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    pub fn push(&mut self, s: String) {
        self.codes.push(s)
    }
}

trait CodeParser {
    fn parse(code: &str) -> Result<Option<CellSources>>;
}
