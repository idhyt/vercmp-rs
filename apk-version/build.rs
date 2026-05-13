use ws_build;

fn main() {
    ws_build::setup_c_build(&["src/version.c"]);
    ws_build::handle_bindings(
        "src/version.h",
        &[
            "apk_version_compare",
            "APK_VERSION_EQUAL",
            "APK_VERSION_LESS",
            "APK_VERSION_GREATER",
        ],
    );
}
