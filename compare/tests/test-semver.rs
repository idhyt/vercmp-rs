use compare::VersionScheme;
use std::cmp::Ordering;

#[test]
fn test_semver_version_compare() {
    let scheme = VersionScheme::SemVer;
    for (a, x, b) in TEST_CASES {
        match scheme.compare(a, b) {
            Ok(actual) => {
                assert_eq!(
                    actual, *x,
                    "Testing failed: expected {} {:?} {}, but got {:?}",
                    a, x, b, actual
                );
                println!("✅: {} {:?} {}", a, x, b);
            }
            Err(e) => {
                panic!("Failed to compare {} and {}: {}", a, b, e)
            }
        }
    }
}

const TEST_CASES: &[(&str, Ordering, &str)] = &[
    // --- 原有示例 (容错/非标准格式) ---
    ("2.34", Ordering::Greater, "0.1.0_alpha"),
    ("23_foo", Ordering::Greater, "4_beta"),
    // --- 基础版本比较 (Major.Minor.Patch) ---
    ("1.0.0", Ordering::Less, "2.0.0"),
    ("2.0.0", Ordering::Greater, "1.9.9"),
    ("2.1.0", Ordering::Greater, "2.0.0"),
    ("2.1.1", Ordering::Greater, "2.1.0"),
    ("1.2.3", Ordering::Equal, "1.2.3"),
    // --- 预发布版优先级 (Pre-release) ---
    // 规则：发行版 > 预发布版
    ("1.0.0", Ordering::Greater, "1.0.0-alpha"),
    // 规则：字母序比较 (alpha < beta < rc)
    ("1.0.0-alpha", Ordering::Less, "1.0.0-beta"),
    ("1.0.0-beta", Ordering::Less, "1.0.0-rc"),
    ("1.0.0-rc", Ordering::Less, "1.0.0"),
    // 规则：数字序比较 (1 < 2 < 10)
    ("1.0.0-alpha.1", Ordering::Less, "1.0.0-alpha.2"),
    ("1.0.0-alpha.2", Ordering::Less, "1.0.0-alpha.10"),
    // 规则：混合比较 (数字优先级低于非数字)
    ("1.0.0-alpha.2", Ordering::Less, "1.0.0-alpha.beta"),
    // 规则：更多字段优先级更高
    ("1.0.0-alpha", Ordering::Less, "1.0.0-alpha.1"),
    // --- 构建元数据 (Build Metadata) ---
    // 规则：构建元数据在比较时应被忽略
    ("1.0.0+build.1", Ordering::Equal, "1.0.0+build.2"),
    ("1.0.0-alpha+001", Ordering::Equal, "1.0.0-alpha+002"),
    ("1.2.3+20230508", Ordering::Equal, "1.2.3+exp.sha.5114f85"),
    // --- 复杂嵌套比较 ---
    ("1.0.0-alpha.beta", Ordering::Less, "1.0.0-beta.alpha"),
    ("1.0.0-rc.1", Ordering::Greater, "1.0.0-beta.11"),
    // --- 零开头的处理 (SemVer标准中数值位不应有前导零，但解析器需稳健) ---
    ("0.0.1", Ordering::Less, "0.1.0"),
    ("1.0.0", Ordering::Greater, "0.9.9"),
    // --- 边界情况/非正规格式测试 (取决于你的解析器实现) ---
    ("1.0", Ordering::Equal, "1.0.0"),      // 补齐缺失位
    ("v1.2.3", Ordering::Equal, "1.2.3"),   // 忽略前缀 'v'
    ("2.1-alpha", Ordering::Less, "2.1.0"), // 简写预发布
    ("1.2.3.4", Ordering::Equal, "1.2.3"),  // 四位版本号处理，多余的数字应被忽略
];
