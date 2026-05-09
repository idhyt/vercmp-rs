use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=apk-version");
    cc::Build::new()
        .file("src/version.c")
        .compile("apk_version");

    let bindings = bindgen::Builder::default()
        .header("src/version.h")
        .clang_arg("-Isrc/")
        .allowlist_function("apk_version_compare")
        .allowlist_var("APK_VERSION_EQUAL")
        .allowlist_var("APK_VERSION_LESS")
        .allowlist_var("APK_VERSION_GREATER")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
