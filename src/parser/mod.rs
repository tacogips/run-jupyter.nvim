pub mod comment_extractor;
pub mod error;
pub mod python_parser;
pub mod rust_parser;

pub type Result<T> = std::result::Result<T, error::ParserError>;

struct CellSources {
    cell_sources: Vec<CellSource>,
}

struct CellSource {
    codes: Vec<String>,
}

trait Parser {
    fn parse(code: &str) -> CellSources;
}
