use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn compare_version_with_purl_type(
    p: *const c_char,
    a: *const c_char,
    b: *const c_char,
) -> i32 {
    use std::cmp::Ordering;
    use std::ffi::CStr;

    use super::{PurlType, VersionScheme};

    if p.is_null() || a.is_null() || b.is_null() {
        eprintln!("Error: null pointer argument");
        return 100;
    }

    let to_option_str = |s: *const c_char| -> &str {
        match unsafe { CStr::from_ptr(p).to_str() } {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Utf8Error: {}", e);
                ""
            }
        }
    };

    let purl = to_option_str(p);
    let version_a = to_option_str(a);
    let version_b = to_option_str(b);
    if purl.is_empty() || version_a.is_empty() || version_b.is_empty() {
        return 101;
    }

    let version_scheme = if purl.starts_with("pkg") {
        match VersionScheme::from_purl(purl) {
            Ok(scheme) => scheme,
            Err(e) => {
                eprintln!("ParseError: {e}");
                return 102;
            }
        }
    } else {
        PurlType::from_str(purl).version_scheme()
    };

    println!("scheme:   {:?}", version_scheme);
    match version_scheme.compare(version_a, version_b) {
        Ok(ord) => {
            println!("compare:  {} {:?} {}", version_a, ord, version_b);
            match ord {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }
        Err(e) => {
            eprintln!("compare: {}", e);
            103
        }
    }
}
