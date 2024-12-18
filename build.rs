use std::path::Path;
use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let parsers_dir = Path::new(&manifest_dir).join("parsers");

    let languages = [
        "rust", "typescript", "python", "c", "cpp", "javascript",
        "bash", "css", "html", "java", "json", "go", "ruby", 
        "toml", "php", "c_sharp", "elisp", "elixir", "elm", 
        "embedded_template", "lua", "rescript", 
        "solidity", "tsx"
    ];

    let tree_sitter_include = format!("{}/target/debug/build/tree-sitter-*/out", manifest_dir);

    for lang in languages.iter() {
        let lang_dir = parsers_dir.join(lang);
        let mut builder = cc::Build::new();

        builder
            .include(&tree_sitter_include)
            .include(&manifest_dir)
            .include(format!("{}/include", manifest_dir));

        // Special handling for PHP which has two parsers
        if *lang == "php" {
            builder
                .include(&lang_dir.join("php/src"))
                .include(&lang_dir.join("php_only/src"));
            if lang_dir.join("php/src/parser.c").exists() {
                builder.file(&lang_dir.join("php/src/parser.c"));
                builder.file(&lang_dir.join("php/src/scanner.c"));
                builder.file(&lang_dir.join("php_only/src/parser.c"));
                builder.file(&lang_dir.join("php_only/src/scanner.c"));
            }
        }
        // Handle special cases for TypeScript and TSX
        else if *lang == "typescript" || *lang == "tsx" {
            let type_dir = if *lang == "typescript" { "typescript" } else { "tsx" };
            let src_path = lang_dir.join(format!("{}/src", type_dir));
            
            builder.include(&src_path);
            
            let parser_path = src_path.join("parser.c");
            if parser_path.exists() {
                builder.file(&parser_path);
            }
            
            let scanner_path = src_path.join("scanner.c");
            if scanner_path.exists() {
                builder.file(&scanner_path);
            }
        }
        else {
            builder.include(&lang_dir.join("src"));
        }

        // Add main parser source
        let parser_path = lang_dir.join("src/parser.c");
        if parser_path.exists() {
            builder.file(&parser_path);
        }

        // Add scanner if it exists (can be .c or .cc)
        let scanner_c_path = lang_dir.join("src/scanner.c");
        let scanner_cc_path = lang_dir.join("src/scanner.cc");
        
        if scanner_c_path.exists() {
            builder.file(&scanner_c_path);
        } else if scanner_cc_path.exists() {
            builder.cpp(true);
            builder.file(&scanner_cc_path);
        }

        // Build with appropriate flags
        builder
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .compile(&format!("tree-sitter-{}", lang.replace("_", "-")));

        println!("cargo:rerun-if-changed={}", lang_dir.display());
    }

    println!("cargo:rerun-if-changed=build.rs");
}