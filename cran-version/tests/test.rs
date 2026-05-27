use std::cmp::Ordering;

use cran_version::compare_version;

#[test]
fn test_cran_version_compare() {
    for (a, x, b) in TEST_CASES {
        match compare_version(a, b) {
            Ok(actual) => {
                assert_eq!(
                    actual, *x,
                    "Testing failed: expected {} {:?} {}, but got {:?}",
                    a, x, b, actual
                );
                println!("✅: {} {:?} {}", a, x, b);
            }
            Err(e) => {
                panic!("Failed to compare {} and {}, error: {}", a, b, e)
            }
        }
    }
}

const TEST_CASES: &[(&str, Ordering, &str)] = &[
    ("0.01", Ordering::Less, "0.1-0"),
    ("0.01.0", Ordering::Equal, "0.1-0"),
    ("0.9", Ordering::Less, "0.75"),
    ("1.0-0", Ordering::Less, "1.1-0"),
    // ---
    ("0.1.0.0", Ordering::Less, "0.2.0"),
    ("0.1.0", Ordering::Less, "0.2.0.0"),
    ("0.1.0", Ordering::Less, "0.1.1.0"),
    ("0.1.0", Ordering::Less, "0.1.0.0"),
    ("0.1.0", Ordering::Greater, "0.0.1.0"),
    // https://cran.r-project.org/src/contrib/Archive/abctools
    ("0.1-2", Ordering::Less, "0.2-2"),
    ("0.2", Ordering::Less, "0.3-2"),
    ("1.0", Ordering::Less, "1.0.1"),
    ("1.0.2", Ordering::Less, "1.0.3"),
    // https://cran.r-project.org/src/contrib/Archive/AntibodyTiters
    ("0.1.4", Ordering::Less, "0.1.18"),
    // https://cran.r-project.org/src/contrib/Archive/AdaptGauss
    ("1.0", Ordering::Less, "1.1.0"),
    ("1.5", Ordering::Less, "1.5.4"),
    // https://cran.r-project.org/src/contrib/Archive/AcceptanceSampling
    ("0.1-1", Ordering::Less, "0.1-4"),
    ("1.0-0", Ordering::Less, "1.0-1"),
    // https://cran.r-project.org/src/contrib/Archive/DHARMa
    ("0.1.0", Ordering::Less, "0.1.0.0"),
    ("0.3.2.0", Ordering::Greater, "0.3.2"),
    ("0.3.2.0", Ordering::Less, "0.3.3.0"),
    ("0.1.0", Ordering::Less, "0.1.1"),
    ("0.1.1", Ordering::Less, "1.0.0"),
    ("1.0.0", Ordering::Less, "1.1.0"),
    ("1.1.0", Ordering::Less, "1.2.0"),
    ("1.2.0", Ordering::Less, "1.4.1"),
    ("0.9.12", Ordering::Less, "0.9.13"),
    ("0.9.13", Ordering::Less, "0.9.14"),
    ("0.9.14", Ordering::Less, "0.9.15"),
    ("0.9.15", Ordering::Less, "0.9.16"),
    ("0.9.16", Ordering::Less, "0.9.17"),
    ("0.9.17", Ordering::Less, "0.9.18"),
    ("0.9.18", Ordering::Less, "0.9.19"),
    ("0.9.19", Ordering::Less, "0.9.20"),
    ("0.9.20", Ordering::Less, "0.9.21"),
    ("0.9.21", Ordering::Less, "0.9.22"),
    ("0.9.22", Ordering::Less, "1.0"),
    ("1.0", Ordering::Less, "1.1"),
    ("1.1", Ordering::Less, "1.2"),
    ("1.2", Ordering::Less, "1.3"),
    ("1.3", Ordering::Less, "1.4"),
    ("1.4", Ordering::Less, "1.5"),
    ("1.5", Ordering::Less, "1.6"),
    ("1.6", Ordering::Less, "1.6.1"),
    ("1.6.1", Ordering::Less, "1.7.0"),
    ("1.7.0", Ordering::Less, "1.7.1"),
    ("1.7.1", Ordering::Less, "1.7.2"),
    ("1.7.2", Ordering::Less, "1.7.3"),
    ("1.7.3", Ordering::Less, "1.8.0"),
    ("1.8.0", Ordering::Less, "1.8.1"),
    ("1.8.1", Ordering::Less, "1.8.2"),
    ("1.8.2", Ordering::Less, "1.8.3"),
    ("1.8.3", Ordering::Less, "1.8.4"),
    ("1.8.4", Ordering::Less, "1.8.5"),
    ("1.8.5", Ordering::Less, "1.8.6"),
    ("1.8.6", Ordering::Less, "1.8.7"),
    ("0.7.1", Ordering::Less, "1.0.0"),
    ("1.0.0", Ordering::Less, "1.0.1"),
    ("1.0.1", Ordering::Less, "1.1.1"),
    ("1.1.1", Ordering::Less, "1.1.2"),
    ("1.1.2", Ordering::Less, "1.2.1"),
    ("1.2.1", Ordering::Less, "1.2.2"),
    ("0.1.1", Ordering::Less, "0.2.0"),
    ("0.2.0", Ordering::Less, "0.2.1"),
    ("0.2.1", Ordering::Less, "1.0.0"),
    ("0.2", Ordering::Less, "0.4"),
    ("0.4", Ordering::Less, "0.5"),
    ("0.5", Ordering::Less, "0.6"),
    ("0.6", Ordering::Less, "0.7"),
    ("0.7", Ordering::Less, "0.8"),
    ("0.8", Ordering::Less, "0.9"),
    ("0.9", Ordering::Less, "1.0"),
    ("1.2", Ordering::Less, "1.4"),
    ("1.6", Ordering::Less, "1.7"),
    ("1.7", Ordering::Less, "1.8"),
    ("1.8", Ordering::Less, "1.8.0"),
    ("1.8.1", Ordering::Less, "1.9"),
    ("1.9", Ordering::Less, "1.9.0"),
    // empty
    ("", Ordering::Less, ""),
];
