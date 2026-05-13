use std::env;
use std::fs;
use std::path::PathBuf;

fn panic_with_info() -> &'static str {
    panic!(
        "No bindings found! Either:\n\
          1. Run with --features generate-bindings to generate them\n\
          2. Or pre-generate bindings with 'FORCE_GENERATE_BINDINGS=1 cargo build --features generate-bindings'\n\
          3. Or commit src/bindings.rs to the repository"
    );
}

pub fn setup_c_build(c_files: &[&str]) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    for file in c_files {
        let path = PathBuf::from(&manifest_dir).join(file);
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    if !c_files.is_empty() {
        let mut build = cc::Build::new();
        for file in c_files {
            build.file(PathBuf::from(&manifest_dir).join(file));
        }
        let crate_name = env::var("CARGO_PKG_NAME").unwrap();
        build.compile(&format!("{}_native", crate_name.replace("-", "_")));
    }
}

#[allow(unused_variables)]
pub fn handle_bindings(header: &str, allowlist: &[&str]) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_bindings = PathBuf::from(&manifest_dir).join("src/bindings.rs");

    let force_generate = env::var("FORCE_GENERATE_BINDINGS").is_ok();
    let should_generate = force_generate || !src_bindings.exists();

    if should_generate {
        #[cfg(feature = "generate-bindings")]
        {
            generate_bindings(&manifest_dir, header, allowlist, &src_bindings);
        }
        #[cfg(not(feature = "generate-bindings"))]
        {
            panic_with_info();
        }
    } else if src_bindings.exists() {
        println!("cargo:rerun-if-changed={}", src_bindings.display());
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        fs::copy(&src_bindings, out_dir.join("bindings.rs")).expect("Failed to copy bindings");
    } else {
        panic_with_info();
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(manifest_dir: &str, header: &str, allowlist: &[&str], dest: &PathBuf) {
    let header_path = PathBuf::from(manifest_dir).join(header);
    let mut builder = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .clang_arg("-Isrc/");

    for item in allowlist {
        builder = builder.allowlist_item(item);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings");

    fs::copy(&out_path, dest).expect("Failed to save bindings");
}
