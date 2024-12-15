use tree_sitter::Language;

extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_bash() -> Language;
    fn tree_sitter_css() -> Language;
    fn tree_sitter_html() -> Language;
    fn tree_sitter_java() -> Language;
    fn tree_sitter_json() -> Language;
    fn tree_sitter_go() -> Language;
    fn tree_sitter_ruby() -> Language;
    fn tree_sitter_toml() -> Language;
    fn tree_sitter_php() -> Language;
    fn tree_sitter_c_sharp() -> Language;
    fn tree_sitter_elisp() -> Language;
    fn tree_sitter_elixir() -> Language;
    fn tree_sitter_elm() -> Language;
    fn tree_sitter_embedded_template() -> Language;
    fn tree_sitter_lua() -> Language;


    fn tree_sitter_rescript() -> Language;
    fn tree_sitter_solidity() -> Language;

    fn tree_sitter_tsx() -> Language;
}

pub fn get_language(name: &str) -> Option<Language> {
    match name {
        "rust" => Some(unsafe { tree_sitter_rust() }),
        "typescript" => Some(unsafe { tree_sitter_typescript() }),
        "python" => Some(unsafe { tree_sitter_python() }),
        "c" => Some(unsafe { tree_sitter_c() }),
        "cpp" => Some(unsafe { tree_sitter_cpp() }),
        "javascript" => Some(unsafe { tree_sitter_javascript() }),
        "bash" => Some(unsafe { tree_sitter_bash() }),
        "css" => Some(unsafe { tree_sitter_css() }),
        "html" => Some(unsafe { tree_sitter_html() }),
        "java" => Some(unsafe { tree_sitter_java() }),
        "json" => Some(unsafe { tree_sitter_json() }),
        "go" => Some(unsafe { tree_sitter_go() }),
        "ruby" => Some(unsafe { tree_sitter_ruby() }),
        "toml" => Some(unsafe { tree_sitter_toml() }),
        "php" => Some(unsafe { tree_sitter_php() }),
        "c_sharp" => Some(unsafe { tree_sitter_c_sharp() }),
        "elisp" => Some(unsafe { tree_sitter_elisp() }),
        "elixir" => Some(unsafe { tree_sitter_elixir() }),
        "elm" => Some(unsafe { tree_sitter_elm() }),
        "embedded_template" => Some(unsafe { tree_sitter_embedded_template() }),
        "lua" => Some(unsafe { tree_sitter_lua() }),


        "rescript" => Some(unsafe { tree_sitter_rescript() }),
        "solidity" => Some(unsafe { tree_sitter_solidity() }),

        "tsx" => Some(unsafe { tree_sitter_tsx() }),
        _ => None,
    }
}

pub fn supported_languages() -> Vec<&'static str> {
    vec![
        "rust", "typescript", "python", "c", "cpp", "javascript",
        "bash", "css", "html", "java", "json", "go", "ruby",
        "toml", "php", "c_sharp", "elisp", "elixir", "elm",
        "embedded_template", "lua", "rescript",
        "solidity", "tsx"
    ]
}