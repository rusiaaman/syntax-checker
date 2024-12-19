use std::path::Path;
use std::env;

fn main() {
    let mut c_builder = cc::Build::new();
    c_builder
        .flag("-std=c99")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");

    let mut cpp_builder = cc::Build::new();
    cpp_builder
        .cpp(true)
        .flag("-std=c++11")
        .flag("-D_GLIBCXX_USE_CXX11_ABI=1")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");


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
        let (mut c_build, mut cpp_build) = (c_builder.clone(), cpp_builder.clone());

        let common_includes = |b: &mut cc::Build| {
            b.include(&tree_sitter_include)
             .include(&manifest_dir)
             .include(format!("{}/include", manifest_dir));
        };
        common_includes(&mut c_build);
        common_includes(&mut cpp_build);

        // Special handling for PHP which has two parsers
        if *lang == "php" {
            c_build
                .include(&lang_dir.join("php/src"))
                .include(&lang_dir.join("php_only/src"));
            cpp_build
                .include(&lang_dir.join("php/src"))
                .include(&lang_dir.join("php_only/src"));
            if lang_dir.join("php/src/parser.c").exists() {
                c_build.file(&lang_dir.join("php/src/parser.c"));
                c_build.file(&lang_dir.join("php/src/scanner.c"));
                c_build.file(&lang_dir.join("php_only/src/parser.c"));
                c_build.file(&lang_dir.join("php_only/src/scanner.c"));
            }
        }
        // Handle special cases for TypeScript and TSX
        else if *lang == "typescript" || *lang == "tsx" {
            let type_dir = if *lang == "typescript" { "typescript" } else { "tsx" };
            let src_path = lang_dir.join(format!("{}/src", type_dir));
            
            c_build.include(&src_path);
            cpp_build.include(&src_path);
            
            let parser_path = src_path.join("parser.c");
            if parser_path.exists() {
                c_build.file(&parser_path);
            }
            
            let scanner_path = src_path.join("scanner.c");
            if scanner_path.exists() {
                c_build.file(&scanner_path);
            }
        }
        else {
            c_build.include(&lang_dir.join("src"));
            cpp_build.include(&lang_dir.join("src"));
        }

        // Add main parser source
        let parser_path = lang_dir.join("src/parser.c");
        if parser_path.exists() {
            c_build.file(&parser_path);
        }

        // Add scanner if it exists (can be .c or .cc)
        let scanner_c_path = lang_dir.join("src/scanner.c");
        let scanner_cc_path = lang_dir.join("src/scanner.cc");
        
        // Special case: C++ scanners with .c extension
        let cpp_scanner_languages = ["cpp", "c"];
        let force_cpp = cpp_scanner_languages.contains(&lang);
        
        let actual_builder = if scanner_cc_path.exists() || (force_cpp && scanner_c_path.exists()) {
            if scanner_cc_path.exists() {
                cpp_build.file(&scanner_cc_path);
            } else {
                cpp_build.file(&scanner_c_path);
            }
            &mut cpp_build
        } else {
            if scanner_c_path.exists() {
                c_build.file(&scanner_c_path);
            }
            &mut c_build
        };

        // Build with flags that have been set in builder initialization
        let lib_name = format!("tree-sitter-{}", lang.replace("_", "-"));
        if force_cpp {
            println!("cargo:rustc-link-lib=dylib=c++");
            actual_builder.cpp_link_stdlib(Some("c++")).compile(&lib_name);
        } else {
            actual_builder.compile(&lib_name);
        }

        // For C++ libraries, make sure we export the tree-sitter symbols
        if force_cpp {
            println!("cargo:rustc-link-lib=static=tree_sitter_{}", lang.replace("_", "-"));
        } else {
            actual_builder.compile(&format!("tree-sitter-{}", lang.replace("_", "-")));
        }

        println!("cargo:rerun-if-changed={}", lang_dir.display());
    }

    println!("cargo:rerun-if-changed=build.rs");
}