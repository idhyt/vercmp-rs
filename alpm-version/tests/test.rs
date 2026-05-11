use std::cmp::Ordering;

use alpm_version::compare_alpm_versions;

#[test]
fn test_alpm_version_compare() {
    for (a, x, b) in TEST_CASES {
        match compare_alpm_versions(a, b) {
            Some(actual) => {
                assert_eq!(
                    actual, *x,
                    "Testing failed: expected {} {:?} {}, but got {:?}",
                    a, x, b, actual
                );
                println!("✅: {} {:?} {}", a, x, b);
            }
            None => {
                panic!("Failed to compare {} and {}", a, b,)
            }
        }
    }
}

const TEST_CASES: &[(&str, Ordering, &str)] = &[
    // all similar length, no pkgrel
    ("1.5.0", Ordering::Equal, "1.5.0"),
    ("1.5.1", Ordering::Greater, "1.5.0"),
    // mixed length
    ("1.5.1", Ordering::Greater, "1.5"),
    // with pkgrel, simple
    ("1.5.0-1", Ordering::Equal, "1.5.0-1"),
    ("1.5.0-1", Ordering::Less, "1.5.0-2"),
    ("1.5.0-1", Ordering::Less, "1.5.1-1"),
    ("1.5.0-2", Ordering::Less, "1.5.1-1"),
    // with pkgrel, mixed lengths
    ("1.5-1", Ordering::Less, "1.5.1-1"),
    ("1.5-2", Ordering::Less, "1.5.1-1"),
    ("1.5-2", Ordering::Less, "1.5.1-2"),
    // mixed pkgrel inclusion
    ("1.5", Ordering::Equal, "1.5-1"),
    ("1.5-1", Ordering::Equal, "1.5"),
    ("1.1-1", Ordering::Equal, "1.1"),
    ("1.0-1", Ordering::Less, "1.1"),
    ("1.1-1", Ordering::Greater, "1.0"),
    // alphanumeric versions
    ("1.5b-1", Ordering::Less, "1.5-1"),
    ("1.5b", Ordering::Less, "1.5"),
    ("1.5b-1", Ordering::Less, "1.5"),
    ("1.5b", Ordering::Less, "1.5.1"),
    // from the manpage
    ("1.0a", Ordering::Less, "1.0alpha"),
    ("1.0alpha", Ordering::Less, "1.0b"),
    ("1.0b", Ordering::Less, "1.0beta"),
    ("1.0beta", Ordering::Less, "1.0rc"),
    ("1.0rc", Ordering::Less, "1.0"),
    // going crazy? alpha-dotted versions
    ("1.5.a", Ordering::Greater, "1.5"),
    ("1.5.b", Ordering::Greater, "1.5.a"),
    ("1.5.1", Ordering::Greater, "1.5.b"),
    // alpha dots and dashes
    ("1.5.b-1", Ordering::Equal, "1.5.b"),
    ("1.5-1", Ordering::Less, "1.5.b"),
    // same/similar content, differing separators
    ("2.0", Ordering::Equal, "2_0"),
    ("2.0_a", Ordering::Equal, "2_0.a"),
    ("2.0a", Ordering::Less, "2.0.a"),
    ("2___a", Ordering::Greater, "2_a"),
    // epoch included version comparisons
    ("0:1.0", Ordering::Equal, "0:1.0"),
    ("0:1.0", Ordering::Less, "0:1.1"),
    ("1:1.0", Ordering::Greater, "0:1.0"),
    ("1:1.0", Ordering::Greater, "0:1.1"),
    ("1:1.0", Ordering::Less, "2:1.1"),
    // epoch + sometimes present pkgrel
    ("1:1.0", Ordering::Greater, "0:1.0-1"),
    ("1:1.0-1", Ordering::Greater, "0:1.1-1"),
    // epoch included on one version
    ("0:1.0", Ordering::Equal, "1.0"),
    ("0:1.0", Ordering::Less, "1.1"),
    ("0:1.1", Ordering::Greater, "1.0"),
    ("1:1.0", Ordering::Greater, "1.0"),
    ("1:1.0", Ordering::Greater, "1.1"),
    ("1:1.1", Ordering::Greater, "1.1"),
];
