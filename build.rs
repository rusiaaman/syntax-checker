use std::path::Path;
use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);
    let tree_sitter_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let languages = [
        ("rust", "https://github.com/tree-sitter/tree-sitter-rust.git"),
        ("typescript", "https://github.com/tree-sitter/tree-sitter-typescript.git"),
        ("python", "https://github.com/tree-sitter/tree-sitter-python.git"),
        ("c", "https://github.com/tree-sitter/tree-sitter-c.git"),
        ("cpp", "https://github.com/tree-sitter/tree-sitter-cpp.git"),
        ("javascript", "https://github.com/tree-sitter/tree-sitter-javascript.git"),
    ];

    let tree_sitter_include = format!("{}/target/debug/build/tree-sitter-*/out", tree_sitter_dir);

    for (lang, repo_url) in languages.iter() {
        let lang_dir = dest_path.join(format!("parsers-{}", lang));
        std::fs::create_dir_all(&lang_dir).expect("Failed to create parser directory");

        let parser_c_path = lang_dir.join("src/parser.c");
        let external_scanner_path = lang_dir.join("src/scanner.c");
        let typescript_parser_c_path = lang_dir.join("typescript/src/parser.c");
        let typescript_scanner_c_path = lang_dir.join("typescript/src/scanner.c");

        if !parser_c_path.exists() && !typescript_parser_c_path.exists() {
            let clone_status = Command::new("git")
                .args(&["clone", "--depth=1", repo_url, lang_dir.to_str().unwrap()])
                .status()
                .expect("Failed to clone parser repository");

            if !clone_status.success() {
                panic!("Failed to clone {} parser", lang);
            }
        }

        let mut builder = cc::Build::new();
        builder
            .include(lang_dir.join("src"))
            .include(lang_dir.join("typescript/src"))
            .include(&tree_sitter_dir)
            .include(format!("{}/include", tree_sitter_dir))
            .include(&tree_sitter_include);

        // Add parser source
        if parser_c_path.exists() {
            builder.file(parser_c_path);
        }
        if typescript_parser_c_path.exists() {
            builder.file(typescript_parser_c_path);
        }

        // Add external scanner if exists
        if external_scanner_path.exists() {
            builder.file(external_scanner_path);
        }
        if typescript_scanner_c_path.exists() {
            builder.file(typescript_scanner_c_path);
        }

        builder
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-but-set-variable")
            .compile(&format!("tree-sitter-{}", lang));

        println!("cargo:rustc-link-search=native={}", dest_path.display());
        println!("cargo:rustc-link-lib=static=tree-sitter-{}", lang);
    }

    println!("cargo:rerun-if-changed=build.rs");
}