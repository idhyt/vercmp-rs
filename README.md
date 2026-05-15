# vercmp-rs

Comprehensive version parsing and comparison library that faithfully implements the versioning rules of all Package URL (purl) ecosystem types (SemVer, RPM, APK, DEB, ALPM, etc.), enabling correct ordering and operator-based matching across packaging formats.

```rust
use vercmp_rs::compare::{PurlType, VersionScheme};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <purl> <version1> <version2>", args[0]);
        println!("Example: {} pkg:cargo/curl 1.0.0 2.0.0", args[0]);
        return;
    }
    let version_scheme = if args[1].starts_with("pkg") {
        VersionScheme::from_purl(&args[1]).unwrap_or_else(|e| {
            println!("compare:  {e}");
            std::process::exit(1);
        })
    } else {
        PurlType::from_str(&args[1]).version_scheme()
    };

    let version1 = &args[2];
    let version2 = &args[3];
    println!("scheme:   {:?}", version_scheme);
    match version_scheme.compare(version1, version2) {
        Ok(ord) => println!("compare:  {} {:?} {}", version1, ord, version2),
        Err(e) => println!("compare:  {}", e),
    }
}
```

or usage with vercmp-rs cli:

```bash
> cargo build -p vercmp-rs --release

> ./target/release/vercmp-rs pkg:golang/google.golang.org/genproto 0.1.1.alpha 0.1.1
scheme:   Semantic
compare:  0.1.1.alpha Less 0.1.1

> ./target/release/vercmp-rs pkg:rpm/xx 3.99.5final.SP07 3.99.5final.SP10
scheme:   Rpm
compare:  3.99.5final.SP07 Less 3.99.5final.SP10

> ./target/release/vercmp-rs apk 3.10.18-r1 3.10.10b-r1
scheme:   Apk
compare:  3.10.18-r1 Greater 3.10.10b-r1

> ./target/release/vercmp-rs deb 1:1.0 0:2.0
scheme:   Deb
compare:  1:1.0 Greater 0:2.0

> ./target/release/vercmp-rs alpm 1.0beta 1.0rc
scheme:   Alpm
compare:  1.0beta Less 1.0rc
```

# building

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

cross build all targets

```bash
cargo clean && cargo build --features generate-bindings && (x() { rm -rf ./target/release && cross build --release --target "$1"; }; x x86_64-unknown-linux-musl && x aarch64-unknown-linux-musl && x x86_64-apple-darwin && x aarch64-apple-darwin && x x86_64-pc-windows-gnu)
```

show the binary info

```bash
find target/*/release/ -type f \( -name "vercmp-rs" -o -name "vercmp-rs.exe" \) -exec sh -c 'echo "=== {} ===" && echo "Size: $(ls -lh "{}" | awk "{print \$5}")" && echo "Type: $(file -b "{}")" && echo "Dependencies:" && ldd "{}" 2>&1; echo' \;
```

run testcase

```bash
cargo test --workspace -- --show-output
```
