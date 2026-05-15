mod version;

use std::cmp::Ordering;
pub use version::new_artifact_version;

pub fn compare_version(a: &str, b: &str) -> Result<Ordering, String> {
    let v1 = new_artifact_version(a);
    let v2 = new_artifact_version(b);
    Ok(v1.cmp(&v2))
}
