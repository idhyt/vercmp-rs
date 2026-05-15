//! This is just all of maven's test cases.
//! https://github.com/apache/maven/blob/master/compat/maven-artifact/src/test/java/org/apache/maven/artifact/versioning/ComparableVersionTest.java

use maven_version::new_artifact_version;
use std::cmp::Ordering;

fn assert_versions_equal(s: &str, t: &str) {
    let c1 = new_artifact_version(s);
    let c2 = new_artifact_version(t);
    assert_eq!(c1.cmp(&c2), Ordering::Equal, "{:?} === {:?}", c1, c2);
    assert_eq!(c2.cmp(&c1), Ordering::Equal, "{:?} === {:?}", c2, c1);
    assert_eq!(c1, c2, "{:?} == {:?}", c1, c2);
    assert_eq!(c2, c1, "{:?} == {:?}", c2, c1);
    println!("✅: {} == {}", s, t);
}

fn assert_versions_ordered(s: &str, t: &str) {
    let c1 = new_artifact_version(s);
    let c2 = new_artifact_version(t);
    assert_eq!(c1.cmp(&c2), Ordering::Less, "{:?} < {:?}", c1, c2);
    assert_eq!(c2.cmp(&c1), Ordering::Greater, "{:?} > {:?}", c2, c1);
    assert_ne!(c1, c2, "{:?} != {:?}", c1, c2);
    assert_ne!(c2, c1, "{:?} != {:?}", c2, c1);
    println!("✅: {} < {}", s, t);
}

fn assert_version_list_ordered(v: Vec<&str>) {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            match v[i].cmp(&v[j]) {
                Ordering::Less => assert_versions_ordered(v[i], v[j]),
                Ordering::Equal => assert_versions_equal(v[i], v[j]),
                Ordering::Greater => assert_versions_ordered(v[i], v[j]),
            };
        }
    }
}

fn assert_version_list_equal(v: Vec<&str>) {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            assert_versions_equal(v[i], v[j]);
        }
    }
}

#[test]
fn test_versions_qualifier() {
    let versions = vec![
        "1-alpha2snapshot",
        "1-alpha2",
        "1-alpha-123",
        "1-beta-2",
        "1-beta123",
        "1-m2",
        "1-m11",
        "1-rc",
        "1-cr2",
        "1-rc123",
        "1-SNAPSHOT",
        "1",
        "1-sp",
        "1-sp2",
        "1-sp123",
        "1-abc",
        "1-def",
        "1-pom-1",
        "1-1-snapshot",
        "1-1",
        "1-2",
        "1-123",
    ];

    assert_version_list_ordered(versions);
}

#[test]
fn test_versions_number() {
    let versions = vec![
        "2.0", "2-1", "2.0.a", "2.0.0.a", "2.0.2", "2.0.123", "2.1.0", "2.1-a", "2.1b", "2.1-c",
        "2.1-1", "2.1.0.1", "2.2", "2.123", "11.a2", "11.a11", "11.b2", "11.b11", "11.m2",
        "11.m11", "11", "11.a", "11b", "11c", "11m",
    ];

    assert_version_list_ordered(versions);
}

#[test]
fn test_versions_equal() {
    assert_versions_equal("1", "1");
    assert_versions_equal("1", "1.0");
    assert_versions_equal("1", "1.0.0");
    assert_versions_equal("1.0", "1.0.0");
    assert_versions_equal("1", "1-0");
    assert_versions_equal("1", "1.0-0");
    assert_versions_equal("1.0", "1.0-0");

    assert_versions_equal("1a", "1-a");
    assert_versions_equal("1a", "1.0-a");
    assert_versions_equal("1a", "1.0.0-a");
    assert_versions_equal("1.0a", "1-a");
    assert_versions_equal("1.0.0a", "1-a");
    assert_versions_equal("1x", "1-x");
    assert_versions_equal("1x", "1.0-x");
    assert_versions_equal("1.0x", "1-x");
    assert_versions_equal("1.0.0x", "1-x");

    assert_versions_equal("1ga", "1");
    assert_versions_equal("1release", "1");
    assert_versions_equal("1b2", "1-beta-2");
    assert_versions_equal("1m3", "1-milestone-3");
    assert_versions_equal("1X", "1x");
    assert_versions_equal("1A", "1a");
    assert_versions_equal("1B", "1b");
    assert_versions_equal("1M", "1m");
    assert_versions_equal("1Ga", "1");
    assert_versions_equal("1GA", "1");
    assert_versions_equal("1RELEASE", "1");
    assert_versions_equal("1release", "1");
    assert_versions_equal("1RELeaSE", "1");
    assert_versions_equal("1Final", "1");
    assert_versions_equal("1FinaL", "1");
    assert_versions_equal("1FINAL", "1");
    assert_versions_equal("1Cr", "1Rc");
    assert_versions_equal("1cR", "1rC");
    assert_versions_equal("1m3", "1Milestone3");
    assert_versions_equal("1m3", "1MileStone3");
    assert_versions_equal("1m3", "1MILESTONE3");
}

#[test]
fn test_version_order() {
    assert_versions_ordered("1", "2");
    assert_versions_ordered("1.5", "2");
    assert_versions_ordered("1", "2.5");
    assert_versions_ordered("1.0", "1.1");
    assert_versions_ordered("1.1", "1.2");
    assert_versions_ordered("1.0.0", "1.1");
    assert_versions_ordered("1.0.1", "1.1");
    assert_versions_ordered("1.1", "1.2.0");

    assert_versions_ordered("1.0-alpha-1", "1.0");
    assert_versions_ordered("1.0-alpha-1", "1.0-alpha-2");
    assert_versions_ordered("1.0-alpha-1", "1.0-beta-1");

    assert_versions_ordered("1.0-beta-1", "1.0-SNAPSHOT");
    assert_versions_ordered("1.0-SNAPSHOT", "1.0");
    assert_versions_ordered("1.0-alpha-1-SNAPSHOT", "1.0-alpha-1");

    assert_versions_ordered("1.0", "1.0-1");
    assert_versions_ordered("1.0-1", "1.0-2");
    assert_versions_ordered("1.0.0", "1.0-1");

    assert_versions_ordered("2.0-1", "2.0.1");
    assert_versions_ordered("2.0.1-klm", "2.0.1-lmn");
    assert_versions_ordered("2.0.1", "2.0.1-xyz");

    assert_versions_ordered("2.0.1", "2.0.1-123");
    assert_versions_ordered("2.0.1-xyz", "2.0.1-123");
}

// Some *very* maven specific test cases which we nonetheless include
// Check the maven source code for more info
#[test]
fn test_mng_5568() {
    let a = "6.1.0";
    let b = "6.1.0-rc3";
    let c = "6.1H.5-beta";

    assert_versions_ordered(b, a);
    assert_versions_ordered(b, c);
    assert_versions_ordered(a, c);
}

#[test]
fn test_mng_6572() {
    let a = "20190126.230843";
    let b = "1234567890.12345";
    let c = "123456789012345.1H.5-beta";
    let d = "12345678901234567890.1H.5-beta";

    assert_versions_ordered(a, b);
    assert_versions_ordered(b, c);
    assert_versions_ordered(a, c);
    assert_versions_ordered(c, d);
    assert_versions_ordered(b, d);
    assert_versions_ordered(a, d);
}

#[test]
fn test_leading_zeroes() {
    let ones = vec![
        "0000000000000000001",
        "000000000000000001",
        "00000000000000001",
        "0000000000000001",
        "000000000000001",
        "00000000000001",
        "0000000000001",
        "000000000001",
        "00000000001",
        "0000000001",
        "000000001",
        "00000001",
        "0000001",
        "000001",
        "00001",
        "0001",
        "001",
        "01",
        "1",
    ];

    let zeroes = vec![
        "0000000000000000000",
        "000000000000000000",
        "00000000000000000",
        "0000000000000000",
        "000000000000000",
        "00000000000000",
        "0000000000000",
        "000000000000",
        "00000000000",
        "0000000000",
        "000000000",
        "00000000",
        "0000000",
        "000000",
        "00000",
        "0000",
        "000",
        "00",
        "0",
    ];

    assert_version_list_equal(ones);
    assert_version_list_equal(zeroes);
}

#[test]
fn test_mng_6964() {
    let a = "1.0-alpha";
    let b = "1.0-beta";
    let c = "1";
    assert_versions_ordered(a, c);
    assert_versions_ordered(b, c);
    assert_versions_ordered(a, b);
}
