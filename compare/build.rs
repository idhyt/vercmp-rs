#[cfg(feature = "generate-capi")]
fn main() {
    use std::env;
    use std::path::PathBuf;
    extern crate cbindgen;

    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let name = "vercmp.h";
    let header = crate_dir.join("include").join(name);
    let config = crate_dir.join("cbindgen.toml");

    cbindgen::generate_with_config(&crate_dir, cbindgen::Config::from_file(config).unwrap())
        .expect("Unable to generate bindings")
        .write_to_file(&header);
}

#[cfg(not(feature = "generate-capi"))]
fn main() {}
