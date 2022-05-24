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
    fn parse(code: &str) -> Result<Option<CellSources>> {
        let mut sources = CellSources::default();
        let mut current_source = CellSource::default();
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
                            let comment = each_child.utf8_text(code.as_bytes())?;
                            let mut comment_interpreter = CommentInterpreter::new(comment);
                            while let Some(comment_ope) = comment_interpreter.next()? {
                                match comment_ope {
                                    CommentOperator::Command(command) => {
                                        let command = if kind == Kind::BlockComment {
                                            remove_trailing_block_comment_delimiter(command)
                                        } else {
                                            command
                                        };

                                        current_source.push(command)
                                    }
                                    CommentOperator::Separator => {
                                        if !current_source.is_empty() {
                                            sources.push(current_source);
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
                    sources.push(current_source);
                }
                Ok(Some(sources))
            }
        }
    }
}

//fn main() {
//    //let julia_lang = unsafe { tree_sitter_julia() };
//    //let rust_lang = unsafe { tree_sitter_rust() };
//    let mut parser = Parser::new();
//    //parser.set_language(python_lang).unwrap();
//    //parser.set_language(python_lang).unwrap();
//    ////parser.set_language(julia_lang).unwrap();
//    //parser.set_language(rust_lang).unwrap();
//    let code = r#"
//
//### aa
//### bb
//def some(a):
//    """ sss """
//    print("----",a)
//
//    "#;
//
//    let tree = parser.parse(code, None).unwrap();
//
//    //println!("{:?}", tree);
//    let root_node = tree.root_node();
//    //println!("{:?}", root_node.to_sexp());
//    //println!("{:?}", root_node.start_position());
//
//    //println!("{:?}", root_node.child(0).unwrap().kind());
//    //println!("{:?}", root_node.utf8_text(code.as_bytes()));
//    let mut cursor = root_node.walk();
//
//    let mut children = root_node.children(&mut cursor);
//    for each in children {
//        each.is_named();
//
//        println!("- -------- -------  ");
//        println!("kind {:?}", each.kind());
//        println!("range {:?}", each.range());
//        println!("start {:?}", each.start_position());
//        println!("end {:?}", each.end_position());
//        println!("kind id {:?}", each.kind_id());
//        println!("text {:?}", each.utf8_text(code.as_bytes()));
//        //println!("{:?}", each);
//    }
//
//    //println!("{:?}", cursor.field_name());
//    //println!("{:?}", cursor.children());
//    //while let Some(a) = root_node.walk() {
//    //    println!("{:?}", a);
//    //}
//}
