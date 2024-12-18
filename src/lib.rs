use pyo3::prelude::*;
use tree_sitter::Parser;
use std::collections::HashSet;

extern "C" {
    fn tree_sitter_bash() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_tsx() -> Language;
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
    errors: Vec<(usize, usize)>,  // (line, column) tuples
    #[pyo3(get)]
    description: String,  // Aggregated error descriptions
}

#[pyfunction]
fn check_syntax(program_extension: &str, program_content: &str) -> PyResult<Output> {
    let mut parser = Parser::new();
    
    // Get the appropriate language parser
    let language: Language = match program_extension {
        "py" => unsafe { tree_sitter_python() },
        "js" => unsafe { tree_sitter_javascript() },
        "ts" => unsafe { tree_sitter_typescript() },
        "tsx" => unsafe { tree_sitter_tsx() },
        "jsx" => unsafe { tree_sitter_javascript() },
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
    let mut descriptions = Vec::new();
    let root_node = tree.root_node();
    
    // Use HashSet to deduplicate error positions
    let mut seen_errors = HashSet::new();
    
    // Traverse the syntax tree to find ERROR nodes
    fn traverse_node(
        node: tree_sitter::Node,
        errors: &mut Vec<(usize, usize)>,
        descriptions: &mut Vec<String>,
        seen: &mut HashSet<(usize, usize)>
    ) {
        // Only process actual error nodes, not nodes that contain errors
        if node.is_error() || node.is_missing() {
            let start = node.start_position();
            let error_pos = (start.row + 1, start.column + 1);
            
            // Only add error if we haven't seen it before
            if !seen.contains(&error_pos) {
                errors.push(error_pos);
                seen.insert(error_pos);
                
                // Generate error description
                let error_type = if node.is_missing() {
                    format!("Missing {}", node.kind())
                } else {
                    format!("Syntax error: unexpected {}", node.kind())
                };
                let context = node.parent()
                    .map(|p| format!(" in {}", p.kind()))
                    .unwrap_or_else(|| String::new());
                descriptions.push(format!("Line {}, Column {}: {}{}", 
                    start.row + 1, start.column + 1, error_type, context));
            }
            
            // For error nodes that span multiple lines, also mark the end position
            let end = node.end_position();
            if end.row > start.row {
                let end_pos = (end.row + 1, end.column + 1);
                if !seen.contains(&end_pos) {
                    errors.push(end_pos);
                    seen.insert(end_pos);
                }
            }
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            traverse_node(child, errors, descriptions, seen);
        }
    }
    
    traverse_node(root_node, &mut errors, &mut descriptions, &mut seen_errors);
    
    let description = if descriptions.is_empty() {
        String::new()
    } else {
        descriptions.join("\n")
    };
    
    Ok(Output { errors, description })
}

#[pymodule]
fn syntax_checker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_syntax, m)?)?;
    m.add_class::<Output>()?;
    Ok(())
}
