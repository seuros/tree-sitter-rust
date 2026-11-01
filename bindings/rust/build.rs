fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = if std::path::Path::new(&manifest_dir).join("src").exists() {
        std::path::PathBuf::from(&manifest_dir).join("src")
    } else if std::path::Path::new(&manifest_dir).join("../../src").exists() {
        std::path::PathBuf::from(&manifest_dir).join("../../src")
    } else {
        panic!("Could not find src directory");
    };

    let mut cc = cc::Build::new();
    cc.std("c11").include(&src_dir);

    #[cfg(target_os = "freebsd")]
    cc.flag("-fPIC").flag("-fvisibility=default");

    #[cfg(target_env = "msvc")]
    cc.flag("-utf-8");

    cc.file(src_dir.join("parser.c"));
    cc.file(src_dir.join("scanner.c"));

    cc.compile("tree_sitter_rust");
}
