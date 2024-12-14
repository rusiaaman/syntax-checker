use tree_sitter::{Tree, Node, TreeCursor};

/// An error detail with its position
#[derive(Debug)]
pub struct ErrorDetail {
    pub start_byte: usize,
    pub end_byte: usize,
    pub error_type: String,
}

pub fn find_errors(tree: &Tree) -> Vec<ErrorDetail> {
    let mut errors = Vec::new();
    let mut cursor = tree.root_node().walk();

    visit_nodes(&mut cursor, &mut |node| {
        if node.is_error() || node.is_missing() {
            errors.push(ErrorDetail {
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
                error_type: node.kind().to_string(),
            });
        }
    });

    errors
}

fn visit_nodes<F: FnMut(Node)>(cursor: &mut TreeCursor, f: &mut F) {
    loop {
        let node = cursor.node();
        f(node);

        if cursor.goto_first_child() {
            visit_nodes(cursor, f);
            cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

/// Convert byte offsets to line and column
pub fn byte_to_line_col(source: &str, byte_offset: usize) -> (usize, usize) {
    let bytes = source.as_bytes();
    let line = bytes[..byte_offset].iter().filter(|&&c| c == b'\n').count();
    let line_start = bytes[..byte_offset].iter().rposition(|&c| c == b'\n').map(|pos| pos + 1).unwrap_or(0);
    let col = byte_offset - line_start;
    (line + 1, col + 1)
}