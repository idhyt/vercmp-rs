build with `generate-bindings` feature

```bash
cargo build --features generate-bindings
```

cross build with `release` profile

```bash
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-pc-windows-gnu
```

use the cli to compare versions

```bash
> ./target/release/compare --help
> ./target/release/compare pkg:golang/google.golang.org/genproto 0.1.1.alpha 0.1.1
scheme:   Semantic
compare:  0.1.1.alpha Less 0.1.1
```

cross build all targets

```bash
cargo clean && cargo build --features generate-bindings && (x() { rm -rf ./target/release && cross build --release --target "$1"; }; x x86_64-unknown-linux-musl && x aarch64-unknown-linux-musl && x x86_64-apple-darwin && x aarch64-apple-darwin && x x86_64-pc-windows-gnu)
```

show the binary info

```bash
find target/*/release/ -type f \( -name "compare" -o -name "compare.exe" \) -exec sh -c 'echo "=== {} ===" && echo "Size: $(ls -lh "{}" | awk "{print \$5}")" && echo "Type: $(file -b "{}")" && echo "Dependencies:" && ldd "{}" 2>&1; echo' \;
```
