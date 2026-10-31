fn main() {
    // Find src directory - handle both workspace root and published crate layouts
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = if std::path::Path::new(&manifest_dir).join("src").exists() {
        std::path::PathBuf::from(&manifest_dir).join("src")
    } else if std::path::Path::new(&manifest_dir).join("../../src").exists() {
        std::path::PathBuf::from(&manifest_dir).join("../../src")
    } else {
        panic!("Could not find src directory for tree-sitter-rust");
    };

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(&src_dir);

    // FreeBSD-specific flags for proper symbol export
    #[cfg(target_os = "freebsd")]
    {
        c_config.flag("-fPIC");
    }

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
    if !parser_path.exists() {
        panic!("parser.c not found at {:?}", parser_path);
    }
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let scanner_path = src_dir.join("scanner.c");
    if !scanner_path.exists() {
        panic!("scanner.c not found at {:?}", scanner_path);
    }
    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    // Set cargo link search path to help with symbol resolution
    println!("cargo:rustc-link-search=native={}",
             std::env::var("OUT_DIR").unwrap());

    c_config.compile("tree-sitter-rust");
}
