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
