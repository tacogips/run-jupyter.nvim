use tree_sitter::{Language, Parser};
extern "C" {
    fn tree_sitter_python() -> Language;
}

fn main() {
    let lang = unsafe { tree_sitter_python() };
    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();
    let code = r#"

### aa
def some(a):
    """ sss """
    print("----",a)

    "#;

    let tree = parser.parse(code, None).unwrap();

    //println!("{:?}", tree);
    let root_node = tree.root_node();
    //println!("{:?}", root_node.to_sexp());
    //println!("{:?}", root_node.start_position());

    //println!("{:?}", root_node.child(0).unwrap().kind());
    //println!("{:?}", root_node.utf8_text(code.as_bytes()));
    let mut cursor = root_node.walk();
    println!("{:?}", cursor.field_name());
    //while let Some(a) = root_node.walk() {
    //    println!("{:?}", a);
    //}
}
