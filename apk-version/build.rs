use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=apkversion");
    // 1. 编译 C 代码为静态库
    cc::Build::new()
        .file("src/version.c")
        .compile("apk_version");

    // 2. 使用 bindgen 生成 Rust 绑定
    let bindings = bindgen::Builder::default()
        .header("src/version.h")
        // 告诉 bindgen 到哪里找头文件
        .clang_arg("-Isrc/")
        // 只生成我们需要的函数和常量
        .allowlist_function("apk_version_compare")
        .allowlist_var("APK_VERSION_EQUAL")
        .allowlist_var("APK_VERSION_LESS")
        .allowlist_var("APK_VERSION_GREATER")
        // 生成的结果
        .generate()
        .expect("Unable to generate bindings");

    // 3. 将生成的绑定写入 OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
