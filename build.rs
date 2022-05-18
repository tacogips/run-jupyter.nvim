fn main() {
    let dir: PathBuf = ["treesitter", "tree-sitter-python", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-python");
}
