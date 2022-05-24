use std::path::{Path, PathBuf};

fn main() {
    build_python();
    build_julia();
    build_rust();
}
fn build_python() {
    let src_dir: PathBuf = ["treesitter", "tree-sitter-python", "src"].iter().collect();

    let mut c_config = cc::Build::new();
    c_config.include(&src_dir);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
    c_config.compile("parser");

    let mut cpp_config = cc::Build::new();
    cpp_config.cpp(true);
    cpp_config.include(&src_dir);
    cpp_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");
    let scanner_path = src_dir.join("scanner.cc");
    cpp_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    cpp_config.compile("scanner");
}

fn build_julia() {}

fn build_rust() {
    let src_dir: PathBuf = ["treesitter", "tree-sitter-rust", "src"].iter().collect();
    let mut config = cc::Build::new();
    config.include(&src_dir);
    config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir.join("parser.c");
    let scanner_path = src_dir.join("scanner.c");
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    config.file(&parser_path);
    config.file(&scanner_path);
    config.compile("parser-scanner");
}
