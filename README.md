# vercmp-rs

A comprehensive version parsing and comparison library that faithfully implements the versioning rules of every [Package URL (purl)](https://github.com/package-url/purl-spec) ecosystem type (SemVer, RPM, APK, DEB, ALPM, and more), enabling correct ordering and operator-based matching across packaging formats.

## Usage

Add the crate you need to your `Cargo.toml`:

```bash
cargo add --git ssh://git@github.com/idhyt/vercmp-rs.git apk-version
```

```rust
fn main() {
    let version_1 = "0.10.2";
    let version_2 = "0.3.1";
    let ord = semantic_version::compare_version(version_1, version_2).unwrap();
    println!("Semantic: {} {:?} {}", version_1, ord, version_2);
}
```

Or add the umbrella crate to work with any ecosystem:

```bash
cargo add --git ssh://git@github.com/idhyt/vercmp-rs.git vercmp-rs
```

```rust
fn main() {
    // Detect the version scheme from a purl
    let purl = "pkg:golang/google.golang.org/genproto";
    // or simply: let purl = "pkg:golang";
    let scheme = vercmp_rs::VersionScheme::from_purl(purl).unwrap();

    // ...or pick one directly from a string
    // let scheme = vercmp_rs::PurlType::from_str("golang").version_scheme();

    let v1 = "0.1.1.alpha";
    let v2 = " 0.1.1";
    println!("{:?}: {} {:?} {}", scheme, v1, scheme.compare(v1, v2).unwrap(), v2);

    // Compare RPM versions
    let scheme = vercmp_rs::PurlType::from_str("rpm").version_scheme();
    let v1 = "3.99.5final.SP07";
    let v2 = "3.99.5final.SP10";
    println!("{:?}: {} {:?} {}", scheme, v1, scheme.compare(v1, v2).unwrap(), v2);
}
```

## CLI

Usage with [vercmp-rs cli example](vercmp/src/main.rs):

```bash
cargo build -p vercmp-rs --release

./target/release/vercmp-rs pkg:golang/google.golang.org/genproto 0.1.1.alpha 0.1.1
# scheme:   Semantic
# compare:  0.1.1.alpha Less 0.1.1

./target/release/vercmp-rs pkg:rpm/xx 3.99.5final.SP07 3.99.5final.SP10
# scheme:   Rpm
# compare:  3.99.5final.SP07 Less 3.99.5final.SP10

./target/release/vercmp-rs apk 3.10.18-r1 3.10.10b-r1
# scheme:   Apk
# compare:  3.10.18-r1 Greater 3.10.10b-r1

./target/release/vercmp-rs deb 1:1.0 0:2.0
# scheme:   Deb
# compare:  1:1.0 Greater 0:2.0

./target/release/vercmp-rs alpm 1.0beta 1.0rc
# scheme:   Alpm
# compare:  1.0beta Less 1.0rc

./target/release/vercmp-rs maven 1m3 1-milestone-3
# scheme:   Maven
# compare:  1m3 Equal 1-milestone-3

./target/release/vercmp-rs opam 1.2.3 1.2.3~preview
# scheme:   Opam
# compare:  1.2.3 Greater 1.2.3~preview

./target/release/vercmp-rs luarocks "3.1.0-1" "3.1.0"
# scheme:   Luarocks
# compare:  3.1.0-1 Equal 3.1.0

./target/release/vercmp-rs cran "1.2-3" "1-2.3"
# scheme:   Cran
# compare:  1.2-3 Equal 1-2.3
```

## Building

Build with the `generate-bindings` feature:

```bash
cargo build --features generate-bindings
```

Cross-build with the `release` profile:

```bash
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-pc-windows-gnu
```

Cross-build all targets:

```bash
cargo clean && cargo build --features generate-bindings && (x() { rm -rf ./target/release && cross build --release --target "$1"; }; x x86_64-unknown-linux-musl && x aarch64-unknown-linux-musl && x x86_64-apple-darwin && x aarch64-apple-darwin && x x86_64-pc-windows-gnu)
```

Inspect built binaries:

```bash
find target/*/release/ -type f \( -name "vercmp-rs" -o -name "vercmp-rs.exe" \) -exec sh -c 'echo "=== {} ===" && echo "Size: $(ls -lh "{}" | awk "{print \$5}")" && echo "Type: $(file -b "{}")" && echo "Dependencies:" && ldd "{}" 2>&1; echo' \;
```

Run tests:

```bash
cargo test -p apk-version -- test_apk_version_compare --show-output
cargo test --workspace -- --show-output
```
