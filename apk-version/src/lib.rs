// 包含 bindgen 生成的代码
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{cmp::Ordering, ffi::CString};

/// 比较两个完整的 APK 版本字符串
pub fn compare_apk_versions(a: &str, b: &str) -> Option<Ordering> {
    let a_cstr = CString::new(a).ok()?;
    let b_cstr = CString::new(b).ok()?;

    let result = unsafe { apk_version_compare(a_cstr.as_ptr(), b_cstr.as_ptr()) } as u32;
    if result == APK_VERSION_EQUAL {
        Some(Ordering::Equal)
    } else if result == APK_VERSION_LESS {
        Some(Ordering::Less)
    } else if result == APK_VERSION_GREATER {
        Some(Ordering::Greater)
    } else {
        None
    }
}

pub fn compare_version(a: &str, b: &str) -> Result<Ordering, String> {
    if let Some(ordering) = compare_apk_versions(a, b) {
        Ok(ordering)
    } else {
        Err("apk version compare unreachable!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(
            compare_apk_versions("2.39.0-r0", "2.39.0-r0"),
            Some(Ordering::Equal)
        );
        assert_eq!(compare_apk_versions("1.0", "2.0"), Some(Ordering::Less));
        assert_eq!(compare_apk_versions("2.0", "1.0"), Some(Ordering::Greater));
    }
}
