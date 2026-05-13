use ws_build;

fn main() {
    ws_build::setup_c_build(&["src/version.c"]);
    ws_build::handle_bindings(
        "src/version.h",
        &[
            "alpm_version_compare",
            "ALPM_VERSION_EQUAL",
            "ALPM_VERSION_LESS",
            "ALPM_VERSION_GREATER",
        ],
    );
}
