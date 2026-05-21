use opam_version::compare_version;
use std::cmp::Ordering;

macro_rules! assert_lt {
    ($x:expr, $y:expr) => {
        assert_eq!(
            compare_version($x, $y),
            Ordering::Less,
            "expected '{}' < '{}'",
            $x,
            $y
        );
        println!("✅: {} < {}", $x, $y);
    };
}
macro_rules! assert_gt {
    ($x:expr, $y:expr) => {
        assert_eq!(
            compare_version($x, $y),
            Ordering::Greater,
            "expected '{}' > '{}'",
            $x,
            $y
        );
        println!("✅: {} > {}", $x, $y);
    };
}
macro_rules! assert_eq_ver {
    ($x:expr, $y:expr) => {
        assert_eq!(
            compare_version($x, $y),
            Ordering::Equal,
            "expected '{}' == '{}'",
            $x,
            $y
        );
        println!("✅: {} == {}", $x, $y);
    };
}

// check with command `opam admin compare-versions v1 v2`
#[test]
fn test_opam_version_compare() {
    debian_policy_examples();
    epoch_like_patterns();
    numerical_parts_simple();
    numerical_parts_different_lengths();
    leading_zeros_ignored();
    tilde_sorts_before_everything();
    letter_vs_non_letter_sorting();
    debian_revision_splitting();
    native_vs_non_native_packages();
    complex_mixed_versions();
    opam_specific_versions();
    opam_git_versions();
    empty_strings();
    equal_strings();
    only_tilde();
    multiple_separators();
    special_characters();

    // https://github.com/mbarbin/opam/blob/master/tests/reftests/admin.test
    assert_lt!("0.0.9", "0.0.10");
    assert_gt!("1.2.3", "1.2.3~preview");
    assert_lt!("1.2.3", "1.2.3.preview");
    assert_lt!("1.2.3", "1.2.3-option");
    assert_gt!("0.2.2", "0.2.0");
    assert_eq_ver!("0.1.0", "0.01.0");
    assert_eq_ver!("0.1.0", "0.1.0");
}

// ========================
// Debian 策略手册范例
// ========================
// #[test]
fn debian_policy_examples() {
    assert_lt!("~~", "~~a");
    assert_lt!("~~a", "~");
    assert_lt!("~", "");
    assert_lt!("", "a");
}

// 注意：原始 opam 实现不处理 epoch，冒号只是普通字符。
// 因此 epoch 示例按实际行为修正。
// #[test]
fn epoch_like_patterns() {
    assert_gt!("1:1.0", "0:2.0"); // '1' > '0'
    assert_lt!("0:1.0", "1.0"); // '0' < '1'，而非相等
    assert_gt!("1.0.0-1", "1.0.0");
    assert_lt!("1.0.0", "1.0.0-1");
    assert_gt!("1.0.0-2", "1.0.0-1");
    assert_lt!("1:1.0.0", "2:0.0.1"); // Epoch 优先级最高
    assert_gt!("2:0.0.1", "1:1.0.0");
    // 优先级: Epoch > Upstream > Revision
    assert_gt!("2:1.0.0", "1:2.0.0"); // 高epoch主导
    assert_gt!("1:2.0.0", "1:1.9.0"); // epoch相同，比较upstream
    assert_gt!("1:1.9.0-2", "1:1.9.0-1"); // upstream相同，比较revision
    assert_lt!("1:1.9.0", "1:1.9.0-1"); // upstream相同，无revision < 有revision
    assert_gt!("1:1.9.0-1", "1:1.9.0"); // 反向验证
}

// ========================
// 数字部分
// ========================
// #[test]
fn numerical_parts_simple() {
    assert_lt!("1.0", "2.0");
    assert_lt!("1.9", "1.10");
    assert_lt!("1.10", "1.10a");
    assert_gt!("2.0", "1.0");
    assert_lt!("0.0.9", "0.0.10");
    assert_lt!("1.0", "1.1");
    assert_eq_ver!("1.0", "1.0");
    assert_lt!("0.2.2", "0.2.10");
    assert_gt!("0.2.10", "0.2.2");
    assert_gt!("0.2.2", "0.2.1");
    assert_lt!("0.2.2", "0.3.0");
    assert_gt!("0.3.0", "0.2.2");

    // 前导零
    assert_eq_ver!("0.1.0", "0.01.0");
    assert_eq_ver!("1.0", "1.00");
    assert_eq_ver!("0.2.0", "0.02.0");
    assert_eq_ver!("0.0.10", "0.00.10");
}

// #[test]
fn numerical_parts_different_lengths() {
    assert_lt!("1.0", "1.0.0");
    assert_lt!("1.0.0", "1.0.1");
    assert_gt!("1.0.1", "1.0.0");
}

// #[test]
fn leading_zeros_ignored() {
    assert_eq_ver!("1.0", "1.00");
    assert_eq_ver!("1.00", "1.0");
    assert_lt!("1.09", "1.10");
    assert_eq_ver!("1.0alpha", "1.0alpha");
    assert_eq_ver!("1.0alpha", "1.0alpha0");
    assert_eq_ver!("1.0alpha0", "1.0alpha");
}

// ========================
// 波浪号
// ========================
// #[test]
fn tilde_sorts_before_everything() {
    assert_lt!("1.0~beta", "1.0");
    assert_lt!("1.0~alpha", "1.0~beta");
    assert_lt!("1.0~beta1", "1.0~beta2");
    assert_lt!("1.0~rc1", "1.0");
    assert_lt!("1.0~", "1.0");
    assert_lt!("1.0~", "1.0~a");
    assert_gt!("1.2.3", "1.2.3~preview");
    assert_gt!("1.0", "1.0~rc1");
    assert_gt!("1.0~preview", "1.0~dev");
    assert_lt!("1.0.0~alpha", "1.0.0");
}

// ========================
// 字母与非字母
// ========================
// #[test]
fn letter_vs_non_letter_sorting() {
    assert_lt!("1.0a", "1.0.0");
    assert_lt!("1.0alpha", "1.0.0");
    assert_gt!("1.0.0", "1.0a");
    assert_gt!("0.33.0-preview.1", "0.33");
    assert_lt!("0.33.0-preview.1", "0.33.0-preview.2");
    assert_lt!("1.0alpha", "1.0beta");
    assert_gt!("1.0beta", "1.0alpha");
}

// ========================
// Debian 修订版本
// ========================
// #[test]
fn debian_revision_splitting() {
    assert_lt!("1.0-1", "1.0-2");
    assert_lt!("1.0-1", "1.0-1a");
    assert_lt!("1.0-1a", "1.0-1b");
    assert_gt!("1.0-2", "1.0-1");
    assert_eq_ver!("1.0-1", "1.0-1");
    assert_lt!("1.2.3", "1.2.3-option");
    assert_eq_ver!("1.0-0", "1.0"); // revision为0，视为与无revision等效
    assert_eq_ver!("1.0-0", "1.0-0");
    assert_lt!("1.0-0", "1.0-1");
    assert_gt!("1.0-1", "1.0-0");
}

// #[test]
fn native_vs_non_native_packages() {
    assert_eq_ver!("1.0", "1.0-0");
    assert_gt!("1.0-1", "1.0");
}

// ========================
// 混合版本
// ========================
// #[test]
fn complex_mixed_versions() {
    assert_lt!("1:2.0~alpha-1", "1:2.0-1");
    assert_lt!("1:2.0-1", "1:2.0-1a");
    assert_lt!("2.4.7-1", "2.4.7-2");
    assert_lt!("2.4.7-2", "2.4.7-3");
    assert_lt!("1:2.4.7-3", "1:3.0-1"); // 冒号作为普通字符
}

// ========================
// opam 常见模式
// ========================
// #[test]
fn opam_specific_versions() {
    assert_lt!("4.08.0", "4.08.1");
    assert_lt!("4.08.0~alpha1", "4.08.0");
    assert_gt!("4.08.0+rc1", "4.08.0"); // '+' 后缀使版本更大
    assert_lt!("4.08.0~rc1", "4.08.0+rc1");
}

// #[test]
fn opam_git_versions() {
    assert_lt!("1.0~git1234", "1.0");
    assert_lt!("1.0~git1234", "1.0-1");
}

// ========================
// 边界情况
// ========================
// #[test]
fn empty_strings() {
    assert_eq_ver!("", ""); // 相同
    assert_eq_ver!("", "0"); // 空等同于 "0"
    assert_lt!("", "a"); // 空小于非数字非~字符
    assert_lt!("", "0.0.1");
    assert_gt!("0.0.1", "");
    assert_eq_ver!("0", "");
    assert_eq_ver!("0", "0");
    assert_eq_ver!("0", "00");
    assert_eq_ver!("00", "0");
}

// #[test]
fn equal_strings() {
    assert_eq_ver!("1.0", "1.0");
    assert_eq_ver!("abc", "abc");
    assert_eq_ver!("1:2.3-4", "1:2.3-4");
    assert_eq_ver!("1.0", "1.0");
    assert_lt!("1.0", "1.1");
    assert_eq_ver!("1.0-0", "1.0");
}

// #[test]
fn only_tilde() {
    assert_lt!("~", "");
    assert_eq_ver!("~", "~"); // 相同
    assert_lt!("~~", "~");
    assert_lt!("~a", "a");
}

// #[test]
fn multiple_separators() {
    assert_lt!("1.2-3-4", "1.2-3-5");
    assert_eq_ver!("1.2-3-4", "1.2-3-4");
    assert_lt!("1.0.0-alpha", "1.0.0-alpha.1");
    assert_gt!("1.0.0-alpha.1", "1.0.0-alpha");
    assert_lt!("1.0.0-beta", "1.0.0-beta.1");
    assert_gt!("1.0.0-beta.1", "1.0.0-beta");
}

// #[test]
fn special_characters() {
    assert_lt!("1.0+dfsg", "1.0+dfsg1");
    assert_lt!("1.0+dfsg1", "1.0+dfsg2");
    assert_lt!("1.0+really2.2-1", "2.3-3");
    assert_lt!("1.0+git", "1.0+stable"); // 特殊字符按ASCII码比较
    assert_gt!("1.0+stable", "1.0+git");
    assert_eq_ver!("1.0+git", "1.0+git");
    assert_lt!("1.0.a", "1.0.b");
    assert_gt!("1.0.b", "1.0.a");
}
