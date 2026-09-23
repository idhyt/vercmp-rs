# vercmp-rs

A workspace of version comparison crates, each faithfully implementing the native versioning rules of a specific [Package URL (purl)](https://github.com/package-url/purl-spec) ecosystem.

## Motivation

Designed for SCA tools that match vulnerabilities via purl. Traditional CPE-based matching produces many false positives due to mismatched version scopes across ecosystems. By using each ecosystem's own version comparison logic, vercmp-rs enables accurate vulnerability matching against purl-identified components.

## Structure

```
vercmp-rs/
├── vercmp/              Umbrella crate — detect the version scheme from a purl and compare
├── semantic-version/    SemVer
├── rpm-version/         RPM
├── apk-version/         Alpine (APK)
├── debian-version/      Debian (DEB)
├── alpm-version/        Arch Linux (ALPM)
├── maven-version/       Maven
├── opam-version/        OCaml (OPAM)
├── luarocks-version/    LuaRocks
├── cran-version/        R (CRAN)
├── ws-build/            Shared build support
└── ...                  TODO implement more
```

Each sub-crate can be used independently, or use the `vercmp` umbrella crate to compare versions across any supported ecosystem with a single API.

See [vercmp/README.md](vercmp/README.md) for usage examples and CLI.
