use compare::{PurlType, VersionScheme};

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
