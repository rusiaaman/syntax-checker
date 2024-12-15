mod parser_reg;
mod error_detector;

use std::env;
use std::fs;
use tree_sitter::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <language> <filepath>", args[0]);
        eprintln!("Supported languages: {}", parser_reg::supported_languages().join(", "));
        std::process::exit(1);
    }

    let language_name = &args[1];
    let filepath = &args[2];

    let language = match parser_reg::get_language(language_name) {
        Some(lang) => lang,
        None => {
            eprintln!("Unsupported language: {}", language_name);
            eprintln!("Supported languages: {}", parser_reg::supported_languages().join(", "));
            std::process::exit(1);
        }
    };

    let source_code = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filepath, e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new();
    parser.set_language(&language).expect("Failed to set language");

    let tree = match parser.parse(&source_code, None) {
        Some(parsed_tree) => parsed_tree,
        None => {
            eprintln!("Failed to parse source code");
            std::process::exit(1);
        }
    };

    let errors = error_detector::find_errors(&tree);

    if errors.is_empty() {
        println!("No syntax errors detected.");
    } else {
        println!("Syntax errors detected:");
        for error in errors {
            let (line, col) = error_detector::byte_to_line_col(&source_code, error.start_byte);
            println!(
                "  - Error at line {}, column {}: Type \"{}\"", 
                line, col, error.error_type
            );
        }
    }
}