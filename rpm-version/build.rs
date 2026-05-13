use ws_build;

fn main() {
    ws_build::setup_c_build(&["src/rpmvercmp.c"]);
    ws_build::handle_bindings(
        "src/version.h",
        &[
            "rpm_version_compare",
            "RPM_VERSION_EQUAL",
            "RPM_VERSION_LESS",
            "RPM_VERSION_GREATER",
        ],
    );
}
