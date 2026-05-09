// mod version;
// pub use version::compare_version;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{cmp::Ordering, ffi::CString};

pub fn compare_rpm_versions(a: &str, b: &str) -> Option<Ordering> {
    let a_cstr = CString::new(a).ok()?;
    let b_cstr = CString::new(b).ok()?;

    let result = unsafe { rpm_version_compare(a_cstr.as_ptr(), b_cstr.as_ptr()) } as u32;
    if result == RPM_VERSION_EQUAL {
        Some(Ordering::Equal)
    } else if result == RPM_VERSION_LESS {
        Some(Ordering::Less)
    } else if result == RPM_VERSION_GREATER {
        Some(Ordering::Greater)
    } else {
        None
    }
}

pub fn compare_version(a: &str, b: &str) -> Result<Ordering, String> {
    if let Some(ordering) = compare_rpm_versions(a, b) {
        Ok(ordering)
    } else {
        Err("apk version compare unreachable!".to_string())
    }
}
