use pyo3::prelude::*;
use tree_sitter::Parser;

extern "C" {
    fn tree_sitter_bash() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
    fn tree_sitter_css() -> Language;
    fn tree_sitter_html() -> Language;
    fn tree_sitter_java() -> Language;
    fn tree_sitter_json() -> Language;
    fn tree_sitter_go() -> Language;
    fn tree_sitter_ruby() -> Language;
    fn tree_sitter_toml() -> Language;
    fn tree_sitter_php() -> Language;
    fn tree_sitter_c_sharp() -> Language;
}

use tree_sitter::Language;

#[pyclass]
struct Output {
    #[pyo3(get)]
    errors: Vec<(usize, usize)>  // (line, column) tuples
}

#[pyfunction]
fn check_syntax(program_name: &str, program_content: &str) -> PyResult<Output> {
    let mut parser = Parser::new();
    
    // Get the appropriate language parser
    let language: Language = match program_name.split('.').last().unwrap_or("") {
        "py" => unsafe { tree_sitter_python() },
        "js" => unsafe { tree_sitter_javascript() },
        "ts" | "tsx" => unsafe { tree_sitter_typescript() },
        "rs" => unsafe { tree_sitter_rust() },
        "c" | "h" => unsafe { tree_sitter_c() },
        "cpp" | "hpp" => unsafe { tree_sitter_cpp() },
        "sh" => unsafe { tree_sitter_bash() },
        "css" => unsafe { tree_sitter_css() },
        "html" | "htm" => unsafe { tree_sitter_html() },
        "java" => unsafe { tree_sitter_java() },
        "json" => unsafe { tree_sitter_json() },
        "go" => unsafe { tree_sitter_go() },
        "rb" => unsafe { tree_sitter_ruby() },
        "toml" => unsafe { tree_sitter_toml() },
        "php" => unsafe { tree_sitter_php() },
        "cs" => unsafe { tree_sitter_c_sharp() },
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Unsupported file extension")),
    };
    
    parser.set_language(&language).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Error loading parser: {}", e))
    })?;
    
    let tree = parser.parse(program_content, None).ok_or_else(|| {
        pyo3::exceptions::PyRuntimeError::new_err("Failed to parse program")
    })?;
    
    let mut errors = Vec::new();
    let root_node = tree.root_node();
    
    // Traverse the syntax tree to find ERROR nodes
    fn traverse_node(node: tree_sitter::Node, errors: &mut Vec<(usize, usize)>) {
        if node.is_error() || node.has_error() {
            let start = node.start_position();
            errors.push((start.row + 1, start.column + 1));
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            traverse_node(child, errors);
        }
    }
    
    traverse_node(root_node, &mut errors);
    
    Ok(Output { errors })
}

#[pymodule]
fn syntax_checker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_syntax, m)?)?;
    m.add_class::<Output>()?;
    Ok(())
}
