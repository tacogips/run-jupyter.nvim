use super::comment_extractor::*;
use super::*;
use tree_sitter::{Language, Parser};

pub type Result<T> = std::result::Result<T, error::ParserError>;
extern "C" {
    fn tree_sitter_rust() -> Language;
}

fn rust_lang() -> Language {
    unsafe { tree_sitter_rust() }
}

#[derive(PartialEq)]
pub enum Kind {
    LineComment,
    BlockComment,
    Other,
}

impl Kind {
    fn from_kind_id(id: u16) -> Kind {
        match id {
            174 => Self::LineComment,
            182 => Self::BlockComment,
            _ => Self::Other,
        }
    }
}

fn remove_trailing_block_comment_delimiter(s: String) -> String {
    if s.ends_with("*/") {
        s[..s.len() - 2].to_string()
    } else {
        s
    }
}

pub struct RustParser;

impl CodeParser for RustParser {
    fn parse(&mut self, code: &str) -> Result<Option<CellSources>> {
        let mut cells = CellSources::default();
        let mut current_source = CellSource::new_code(vec![]);
        let mut parser = Parser::new();
        parser.set_language(rust_lang())?;
        let tree = parser.parse(code, None);
        match tree {
            None => Ok(None),
            Some(tree) => {
                let root_node = tree.root_node();
                let mut cursor = root_node.walk();
                let children = root_node.children(&mut cursor);
                for each_child in children {
                    let kind = Kind::from_kind_id(each_child.kind_id());
                    match kind {
                        Kind::LineComment | Kind::BlockComment => {
                            let comment_str = each_child.utf8_text(code.as_bytes())?;
                            let mut comment_interpreter = CommentInterpreter::new(comment_str);
                            while let Some(comment_ope) = comment_interpreter.next()? {
                                match comment_ope {
                                    CommentOperator::Command(command) => {
                                        let command = if kind == Kind::BlockComment {
                                            remove_trailing_block_comment_delimiter(command)
                                        } else {
                                            command
                                        };

                                        current_source.push(comment_str.to_string());
                                        cells.push(current_source);

                                        current_source = CellSource::new_code(vec![command]);
                                        cells.push(current_source);

                                        current_source = CellSource::new_code(vec![]);
                                    }
                                    CommentOperator::Separator => {
                                        if !current_source.is_empty() {
                                            cells.push(current_source);
                                        }
                                        current_source = CellSource::default();
                                    }
                                }
                            }
                        }
                        Kind::Other => {
                            let code = each_child.utf8_text(code.as_bytes())?;
                            current_source.push(code.to_string());
                        }
                    }
                }

                if !current_source.is_empty() {
                    cells.push(current_source);
                }
                Ok(Some(cells))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::CodeParser;

    #[test]
    fn test_parse_1() {
        let mut parser = RustParser;
        let code = r#"

            mod some;
            fn some(){
                println!("{}","aa");
            }
            "#;

        let parsed = parser.parse(code).unwrap().unwrap();

        let mut sources = CellSources::default();
        sources.push(CellSource::new_code(vec![
            "mod some;".to_string(),
            r#"fn some(){
                println!("{}","aa");
            }"#
            .to_string(),
        ]));

        assert_eq!(parsed, sources);
    }

    #[test]
    fn test_parse_2() {
        let mut parser = RustParser;
        let code = r#"// %% :dep tokio
let val =  "ss";
val"#;

        let parsed = parser.parse(code).unwrap().unwrap();

        let mut sources = CellSources::default();
        sources.push(CellSource::new_code(vec!["// %% :dep tokio".to_string()]));
        sources.push(CellSource::new_code(vec![" :dep tokio".to_string()]));
        sources.push(CellSource::new_code(vec![
            r#"let val =  "ss";"#.to_string(),
            r#"val"#.to_string(),
        ]));

        assert_eq!(parsed, sources);
    }
}
