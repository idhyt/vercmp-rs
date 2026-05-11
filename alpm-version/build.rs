use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=alpm-version");
    cc::Build::new()
        .file("src/version.c")
        .compile("alpm_version");

    let bindings = bindgen::Builder::default()
        .header("src/version.h")
        .clang_arg("-Isrc/")
        .allowlist_function("alpm_version_compare")
        .allowlist_var("ALPM_VERSION_EQUAL")
        .allowlist_var("ALPM_VERSION_LESS")
        .allowlist_var("ALPM_VERSION_GREATER")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
