/* Enable [lib] in the Cargo.toml to build as a shared library
 * cargo build -p vercmp-rs --lib --features generate-capi --release
 * ln -s ../../target/release/libvercmp.dylib ./
 * gcc -o vercmp main.c ./libvercmp.dylib
 * ./vercmp  maven 4.1.5.RELEASE 4.1.5
 */
#include "vercmp.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
    if (argc != 4) {
        printf("Usage: %s <purl_type> <version1> <version2>\n", argv[0]);
        return 1;
    }

    char *purl_type = argv[1];
    char *v1 = argv[2];
    char *v2 = argv[3];
    int result = compare_version_with_purl_type(purl_type, v1, v2);
    if (result == CMP_EQUAL) {
        printf("Result: %d (equal)\n", result);
    } else if (result == CMP_LESS) {
        printf("Result: %d (less)\n", result);
    } else if (result == CMP_GREATER) {
        printf("Result: %d (greater)\n", result);
    } else {
        printf("Result: %d (unknown)\n", result);
        return 1;
    }
    return 0;
}
