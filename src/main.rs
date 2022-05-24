mod parser;
use tree_sitter::{Language, Parser};
extern "C" {
    fn tree_sitter_python() -> Language;
}

extern "C" {
    fn tree_sitter_rust() -> Language;
}
const code: &'static str = r#"


//!

/*
 * some
 * aaa
 */

// line comment
type Result<T> = std::result::Result<T, JupyterApiError>;

pub struct KernelApiClient {
    url: String,
}

const DEFAULT_TIMEOUT_SEC: u64 = 120;

pub fn find_request_result(message: KernelResponse) -> Option<KernelResponse> {
    match message.msg_type {
        MessageType::ExecuteReply | MessageType::ExecuteResult | MessageType::Error => {
            Some(message)
        }
        _ => None,
    }
}

"#;

fn main() {
    //let python_lang = unsafe { tree_sitter_python() };
    //let julia_lang = unsafe { tree_sitter_julia() };
    let rust_lang = unsafe { tree_sitter_rust() };
    let mut parser = Parser::new();
    //parser.set_language(python_lang).unwrap();
    parser.set_language(rust_lang).unwrap();
    //parser.set_language(python_lang).unwrap();
    ////parser.set_language(julia_lang).unwrap();
    //parser.set_language(rust_lang).unwrap();
    //    let code = r#"
    //
    //### aa
    //### bb
    //def some(a):
    //    """ sss """
    //    print("----",a)
    //
    //    "#;

    let tree = parser.parse(code, None).unwrap();

    //println!("{:?}", tree);
    let root_node = tree.root_node();
    //println!("{:?}", root_node.to_sexp());
    //println!("{:?}", root_node.start_position());

    //println!("{:?}", root_node.child(0).unwrap().kind());
    //println!("{:?}", root_node.utf8_text(code.as_bytes()));
    let mut cursor = root_node.walk();

    let mut children = root_node.children(&mut cursor);
    for each in children {
        each.is_named();

        println!("- -------- -------  ");
        println!("kind {:?}", each.kind());
        println!("range {:?}", each.range());
        println!("start {:?}", each.start_position());
        println!("end {:?}", each.end_position());
        println!("kind id {:?}", each.kind_id());
        println!("text {:?}", each.utf8_text(code.as_bytes()));
        //println!("{:?}", each);
    }

    //println!("{:?}", cursor.field_name());
    //println!("{:?}", cursor.children());
    //while let Some(a) = root_node.walk() {
    //    println!("{:?}", a);
    //}
}
