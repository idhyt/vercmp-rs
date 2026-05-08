use compare::VersionScheme;
use std::cmp::Ordering;

#[test]
fn test_debian_version_compare() {
    // 假设你的库中有 Debian 方案，如果没有，请确保逻辑符合 dpkg --compare-versions
    let scheme = VersionScheme::Deb;
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

// dpkg --compare-versions "1...." gt "1." && echo "Greater"
const TEST_CASES: &[(&str, Ordering, &str)] = &[
    // --- 1. 基础数字比较 ---
    ("1.0", Ordering::Equal, "1.0"),
    ("1.0", Ordering::Less, "1.1"),
    ("2.0", Ordering::Greater, "1.9"),
    ("1.0.0", Ordering::Equal, "1.0.0"),
    ("1.10", Ordering::Greater, "1.2"), // 注意：Debian 是按数字段大小比较，不是字符串
    // --- 2. Epoch (时代/纪元) 比较 ---
    // 规则：Epoch 不同则直接决定大小，缺省为 0
    ("1:1.0", Ordering::Greater, "0:2.0"),
    ("1:1.0", Ordering::Greater, "2.0"),
    ("2:0.1", Ordering::Greater, "1:9.9"),
    ("0:1.0", Ordering::Equal, "1.0"),
    // --- 3. Debian Revision (修订号) 比较 ---
    // 规则：最后一个 '-' 之后的部分是修订号
    ("1.0-1", Ordering::Less, "1.0-2"),
    ("1.0-1", Ordering::Less, "1.0.1-1"),
    ("1.0", Ordering::Less, "1.0-1"), // 无修订号视为修订号为空，空 < 非空
    ("1.0-0", Ordering::Equal, "1.0"), // 在某些实现中 -0 等于空，但严格来说空 < 0
    // --- 4. Tilde (~) 的特殊处理 (核心难点) ---
    // 规则：~ 字符排在一切字符之前，包括空字符串。常用于 pre-release
    ("1.0~rc1", Ordering::Less, "1.0"),
    ("1.0~beta", Ordering::Less, "1.0~rc1"),
    ("1.0~rc1", Ordering::Less, "1.0-1"),
    ("1.0-pre~1", Ordering::Less, "1.0-pre"),
    ("2.0~alpha1~exp", Ordering::Less, "2.0~alpha1"),
    // --- 5. 字母与非数字符号比较 ---
    // 规则：字母按 ASCII 比较，但在某些位置非数字字符（如 . +）有特殊权重
    ("1.0a", Ordering::Greater, "1.0"),
    ("1.0-a", Ordering::Less, "1.0-b"),
    ("1.0+ext", Ordering::Greater, "1.0"),
    ("1.0.1", Ordering::Greater, "1.0"),
    // --- 6. 复杂/混合场景 (典型 Debian 软件包写法) ---
    ("1.2.3-1ubuntu1", Ordering::Greater, "1.2.3-1"),
    ("1.2.3-1ubuntu2", Ordering::Greater, "1.2.3-1ubuntu1"),
    ("1.2.3-1~noble1", Ordering::Less, "1.2.3-1"), // Ubuntu 常用 PPA 命名方案
    ("2:1.0-1", Ordering::Greater, "1:2.0-5"),
    // --- 7. 极端情况与容错 ---
    ("001", Ordering::Equal, "1"), // 前导零应被忽略
    ("1.002.0", Ordering::Equal, "1.2.0"),
    ("...1", Ordering::Greater, "1"),   // 非数字在前
    ("1....", Ordering::Greater, "1."), // 多个连续符号
    ("", Ordering::Equal, ""),          // 空字符串比较
    ("~", Ordering::Less, "0"),         // 最小的有效字符
    ("1..2", Ordering::Greater, "1.2"), // . 的 ASCII 比数字小，但两个 . 比一个 . 长度长
    ("1.a", Ordering::Greater, "1.2"),  // 字母的优先级高于所有非字母符号 (A-Z > . + -)
    ("1.0", Ordering::Greater, "1.0~"), // ~ 永远小于空，所以 1.0 (空) > 1.0~
];
