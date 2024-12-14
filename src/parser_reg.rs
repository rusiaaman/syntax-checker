use tree_sitter::Language;

extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
    fn tree_sitter_javascript() -> Language;
}

pub fn get_language(name: &str) -> Option<Language> {
    match name {
        "rust" => Some(unsafe { tree_sitter_rust() }),
        "typescript" => Some(unsafe { tree_sitter_typescript() }),
        "python" => Some(unsafe { tree_sitter_python() }),
        "c" => Some(unsafe { tree_sitter_c() }),
        "cpp" => Some(unsafe { tree_sitter_cpp() }),
        "javascript" => Some(unsafe { tree_sitter_javascript() }),
        _ => None,
    }
}

pub fn supported_languages() -> Vec<&'static str> {
    vec!["rust", "typescript", "python", "c", "cpp", "javascript"]
}