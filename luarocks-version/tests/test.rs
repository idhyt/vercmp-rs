use luarocks_version::{
    ConstraintOp, Version, check_single_constraint, compare_version, match_constraints,
    parse_constraint,
};
use std::cmp::Ordering;

#[test]
fn test_luarocks_version_compare() {
    test_parse_simple_numbers();
    test_parse_with_revision();
    test_parse_revision_edge_cases();
    test_parse_special_markers();
    test_parse_special_markers_case_insensitive();
    test_parse_unknown_words();
    test_parse_delimiters();
    test_parse_tilde_is_not_special();
    test_parse_empty_segments_filtered();
    test_parse_whitespace_trimmed();
    test_parse_error_cases();
    test_compare_equal_versions();
    test_compare_missing_segments_as_zero();
    test_compare_numeric_order();
    test_compare_special_markers_order();
    test_compare_with_revision();
    test_compare_complex_cases();
    test_compare_unknown_words_numeric();
    test_compare_partial_ordering();
    test_parse_constraint_all_operators();
    test_match_constraints_basic_ops();
    test_match_constraints_multiple();
    test_match_constraints_tilde_operator();
    test_match_constraints_with_special_markers();
    test_official_examples_from_docs();
    test_edge_cases_from_luarocks_test_suite();
    test_sorting_consistency();
    test_from_str_trait();
    test_clone_and_debug();
    test_hash_eq_consistency();
    test_long_version_strings();
    test_zero_values();
    test_large_revision_numbers();
    test_mixed_special_and_numbers();
    test_constraint_parsing_edge_cases();
    test_match_constraints_empty();
    test_compare_scm_vs_revision();
}

// ============= 基础解析测试 =============

// #[test]
fn test_parse_simple_numbers() {
    assert_eq!(Version::parse("1").unwrap().parts, vec![1]);
    assert_eq!(Version::parse("1.2.3").unwrap().parts, vec![1, 2, 3]);
    assert_eq!(Version::parse("10.20.30").unwrap().parts, vec![10, 20, 30]);
}

// #[test]
fn test_parse_with_revision() {
    let v = Version::parse("1.0-1").unwrap();
    assert_eq!(v.parts, vec![1, 0]);
    assert_eq!(v.revision, Some(1));

    let v = Version::parse("2.5.1-100").unwrap();
    assert_eq!(v.parts, vec![2, 5, 1]);
    assert_eq!(v.revision, Some(100));
}

// #[test]
fn test_parse_revision_edge_cases() {
    // 末尾-但不是数字，不视为修订号
    let v = Version::parse("1.0-beta").unwrap();
    assert_eq!(v.parts, vec![1, 0, -100_000]); // beta = -100000
    assert_eq!(v.revision, None);

    // 开头-，不视为修订号
    let v = Version::parse("-1").unwrap();
    assert_eq!(v.parts, vec![1]);
    assert_eq!(v.revision, None);

    // 中间-作为分隔符
    let v = Version::parse("1-0-2").unwrap();
    assert_eq!(v.parts, vec![1, 0]);
    assert_eq!(v.revision, Some(2));
}

// #[test]
fn test_parse_special_markers() {
    assert_eq!(
        Version::parse("1.0alpha").unwrap().parts,
        vec![1, 0, -1_000_000]
    );
    assert_eq!(
        Version::parse("1.0beta").unwrap().parts,
        vec![1, 0, -100_000]
    );
    assert_eq!(Version::parse("1.0rc").unwrap().parts, vec![1, 0, -1_000]);
    assert_eq!(Version::parse("1.0pre").unwrap().parts, vec![1, 0, -10_000]);
    assert_eq!(
        Version::parse("1.0scm").unwrap().parts,
        vec![1, 0, 110_000_000]
    );
    assert_eq!(
        Version::parse("1.0dev").unwrap().parts,
        vec![1, 0, 120_000_000]
    );
    assert_eq!(
        Version::parse("1.0cvs").unwrap().parts,
        vec![1, 0, 100_000_000]
    );
}

// #[test]
fn test_parse_special_markers_case_insensitive() {
    assert_eq!(
        Version::parse("1.0BETA").unwrap(),
        Version::parse("1.0beta").unwrap()
    );
    assert_eq!(
        Version::parse("1.0ScM").unwrap(),
        Version::parse("1.0scm").unwrap()
    );
    assert_eq!(
        Version::parse("1.0ALPHA").unwrap(),
        Version::parse("1.0alpha").unwrap()
    );
}

// #[test]
fn test_parse_unknown_words() {
    // 未知单词：首字符 ASCII / 1000（整数除法）
    // 'x'=120 -> 0, 'A'=65 -> 0, 'z'=122 -> 0
    assert_eq!(Version::parse("1.0xyz").unwrap().parts, vec![1, 0, 0]);
    assert_eq!(Version::parse("1.0ABC").unwrap().parts, vec![1, 0, 0]);

    // 单字符未知词
    assert_eq!(Version::parse("1.0x").unwrap().parts, vec![1, 0, 0]); // 'x'=120/1000=0
}

// #[test]
fn test_parse_delimiters() {
    // 支持 . - _ 三种分隔符
    assert_eq!(Version::parse("1.2.3").unwrap().parts, vec![1, 2, 3]);
    // 优先使用最后一个连字符 - 来分离修订版本号
    assert_eq!(Version::parse("1-2-3").unwrap().parts, vec![1, 2]);
    assert_eq!(Version::parse("1_2_3").unwrap().parts, vec![1, 2, 3]);
    assert_eq!(Version::parse("1.2-3_4").unwrap().parts, vec![1, 2, 3, 4]);

    // 混合分隔符
    assert_eq!(
        Version::parse("1.2-3_4.5").unwrap().parts,
        vec![1, 2, 3, 4, 5]
    );
}

// #[test]
fn test_parse_tilde_is_not_special() {
    // ~ 不是分隔符，作为普通字符处理
    // "2.1~beta" -> ["2", "1~beta"] -> [2, 0]（"1~beta"首字符'1'=49/1000=0）
    let v = Version::parse("2.1~beta").unwrap();
    assert_eq!(v.parts, vec![2, 1, -100000]);
}

// #[test]
fn test_parse_empty_segments_filtered() {
    // 连续分隔符产生空段，应被过滤
    assert_eq!(Version::parse("1..2").unwrap().parts, vec![1, 2]);
    assert_eq!(Version::parse("1--2").unwrap().parts, vec![1]);
    assert_eq!(Version::parse("1--2").unwrap().revision, Some(2));
    assert_eq!(Version::parse("1__2").unwrap().parts, vec![1, 2]);
    assert_eq!(Version::parse(".1.2.").unwrap().parts, vec![1, 2]);
}

// #[test]
fn test_parse_whitespace_trimmed() {
    // 解析前会 trim，但版本内部空格会作为未知词
    let v = Version::parse(" 1.2.3 ").unwrap();
    assert_eq!(v.parts, vec![1, 2, 3]);
}

// #[test]
fn test_parse_error_cases() {
    // 空字符串
    assert!(Version::parse("").is_err());

    // 只有分隔符
    let v = Version::parse(".").unwrap();
    assert_eq!(v.parts, vec![]);
}

// ============= 比较逻辑测试 =============

// #[test]
fn test_compare_equal_versions() {
    assert_eq!(
        Version::parse("1.2.3").unwrap(),
        Version::parse("1.2.3").unwrap()
    );
    assert_eq!(
        Version::parse("1.2.3")
            .unwrap()
            .compare(&Version::parse("1.2.3").unwrap()),
        Ordering::Equal
    );
}

// #[test]
fn test_compare_missing_segments_as_zero() {
    // 缺失段视为 0
    assert_eq!(compare_version("1.0", "1.0.0").unwrap(), Ordering::Equal);
    assert_eq!(compare_version("1", "1.0.0").unwrap(), Ordering::Equal);
    assert_eq!(compare_version("1.2", "1.2.0").unwrap(), Ordering::Equal);
    assert_eq!(
        compare_version("1.2.3", "1.2.3.0").unwrap(),
        Ordering::Equal
    );
}

// #[test]
fn test_compare_numeric_order() {
    assert!(Version::parse("1.2.3").unwrap() < Version::parse("1.2.4").unwrap());
    assert!(Version::parse("1.2.3").unwrap() < Version::parse("1.3.0").unwrap());
    assert!(Version::parse("1.2.3").unwrap() < Version::parse("2.0.0").unwrap());
    assert!(Version::parse("2.0").unwrap() > Version::parse("1.999").unwrap());
}

// #[test]
fn test_compare_special_markers_order() {
    // 官方顺序：alpha < beta < rc < pre < release < cvs < scm < dev
    let alpha = Version::parse("1.0alpha").unwrap();
    let beta = Version::parse("1.0beta").unwrap();
    let rc = Version::parse("1.0rc").unwrap();
    let pre = Version::parse("1.0pre").unwrap();
    let release = Version::parse("1.0").unwrap();
    let cvs = Version::parse("1.0cvs").unwrap();
    let scm = Version::parse("1.0scm").unwrap();
    let dev = Version::parse("1.0dev").unwrap();
    assert!(alpha < beta && beta < rc && pre < rc && rc < release);
    assert!(release < cvs && cvs < scm && scm < dev);
}

// #[test]
fn test_compare_with_revision() {
    // 关键：修订号仅当双方都有时才比较
    assert_eq!(compare_version("1.0", "1.0-1").unwrap(), Ordering::Equal);
    assert_eq!(compare_version("1.0", "1.0-0").unwrap(), Ordering::Equal);

    assert!(Version::parse("1.0-1").unwrap() < Version::parse("1.0-2").unwrap());
    assert!(Version::parse("1.0-10").unwrap() > Version::parse("1.0-2").unwrap());

    // 不同主版本，修订号不参与
    assert!(Version::parse("1.0-999").unwrap() < Version::parse("2.0-1").unwrap());
}

// #[test]
fn test_compare_complex_cases() {
    // scm 作为数值段 > 普通版本
    assert!(Version::parse("1.0scm").unwrap() > Version::parse("1.0.1").unwrap());
    assert!(Version::parse("1.0.999").unwrap() < Version::parse("1.0scm").unwrap());

    // 带分隔符的 scm
    assert_eq!(
        Version::parse("1.0scm").unwrap(),
        Version::parse("1.0-scm").unwrap()
    );

    // 预发布 < 正式版本
    assert!(Version::parse("1.0beta").unwrap() < Version::parse("1.0").unwrap());
    assert_eq!(
        Version::parse("1.0beta1").unwrap().parts,
        vec![1, 0, -100_000, 1]
    );
}

// #[test]
fn test_compare_unknown_words_numeric() {
    // 未知单词转数值后参与比较
    let v1 = Version::parse("1.0xyz").unwrap(); // [1, 0, 0]
    let v2 = Version::parse("1.0.0").unwrap(); // [1, 0, 0]
    assert_eq!(v1, v2);

    // 不同首字符可能产生不同值（但 ASCII<1000 时都是 0）
    let v1 = Version::parse("1.0a").unwrap(); // 'a'=97/1000=0
    let v2 = Version::parse("1.0z").unwrap(); // 'z'=122/1000=0
    assert_eq!(v1, v2);
}

// #[test]
fn test_compare_partial_ordering() {
    // 测试 PartialOrd trait
    let v1 = Version::parse("1.0").unwrap();
    let v2 = Version::parse("2.0").unwrap();

    assert!(v1 < v2);
    assert!(v2 > v1);
    assert!(v1 <= v2);
    assert!(v2 >= v1);
    assert!(!(v1 > v2));
}

// ============= 约束匹配测试 =============

// #[test]
fn test_parse_constraint_all_operators() {
    assert_eq!(
        parse_constraint("== 1.0").unwrap(),
        (ConstraintOp::Eq, "1.0".to_string())
    );
    assert_eq!(
        parse_constraint("~= 1.0").unwrap(),
        (ConstraintOp::Ne, "1.0".to_string())
    );
    assert_eq!(
        parse_constraint("< 1.0").unwrap(),
        (ConstraintOp::Lt, "1.0".to_string())
    );
    assert_eq!(
        parse_constraint("<= 1.0").unwrap(),
        (ConstraintOp::Le, "1.0".to_string())
    );
    assert_eq!(
        parse_constraint("> 1.0").unwrap(),
        (ConstraintOp::Gt, "1.0".to_string())
    );
    assert_eq!(
        parse_constraint(">= 1.0").unwrap(),
        (ConstraintOp::Ge, "1.0".to_string())
    );
    assert_eq!(
        parse_constraint("~> 1.0").unwrap(),
        (ConstraintOp::Tilde, "1.0".to_string())
    );

    // 无操作符默认为==
    assert_eq!(
        parse_constraint("1.0").unwrap(),
        (ConstraintOp::Eq, "1.0".to_string())
    );

    // 空格处理
    assert_eq!(
        parse_constraint("  >=  1.0  ").unwrap(),
        (ConstraintOp::Ge, "1.0".to_string())
    );
}

// #[test]
fn test_match_constraints_basic_ops() {
    let v = Version::parse("2.1.0").unwrap();

    assert!(match_constraints(
        &v,
        &[(ConstraintOp::Eq, Version::parse("2.1.0").unwrap())]
    ));
    assert!(match_constraints(
        &v,
        &[(ConstraintOp::Ne, Version::parse("2.0.0").unwrap())]
    ));
    assert!(match_constraints(
        &v,
        &[(ConstraintOp::Gt, Version::parse("2.0.0").unwrap())]
    ));
    assert!(match_constraints(
        &v,
        &[(ConstraintOp::Lt, Version::parse("2.2.0").unwrap())]
    ));
    assert!(match_constraints(
        &v,
        &[(ConstraintOp::Ge, Version::parse("2.1.0").unwrap())]
    ));
    assert!(match_constraints(
        &v,
        &[(ConstraintOp::Le, Version::parse("2.1.0").unwrap())]
    ));

    // 失败情况
    assert!(!match_constraints(
        &v,
        &[(ConstraintOp::Eq, Version::parse("2.0.0").unwrap())]
    ));
    assert!(!match_constraints(
        &v,
        &[(ConstraintOp::Lt, Version::parse("2.0.0").unwrap())]
    ));
}

// #[test]
fn test_match_constraints_multiple() {
    let v = Version::parse("2.1.5").unwrap();

    // 多个约束：全部满足才返回 true
    let constraints = vec![
        (ConstraintOp::Ge, Version::parse("2.0.0").unwrap()),
        (ConstraintOp::Lt, Version::parse("2.2.0").unwrap()),
        (ConstraintOp::Ne, Version::parse("2.1.0").unwrap()),
    ];
    assert!(match_constraints(&v, &constraints));

    // 一个不满足就失败
    let constraints = vec![
        (ConstraintOp::Ge, Version::parse("2.0.0").unwrap()),
        (ConstraintOp::Eq, Version::parse("2.1.0").unwrap()), // 不满足
    ];
    assert!(!match_constraints(&v, &constraints));
}

// #[test]
fn test_match_constraints_tilde_operator() {
    // ~> 2 => >= 2, < 3
    assert!(check_single_constraint(
        &Version::parse("2.0").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2").unwrap()
    ));
    assert!(check_single_constraint(
        &Version::parse("2.999").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2").unwrap()
    ));
    assert!(!check_single_constraint(
        &Version::parse("3.0").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2").unwrap()
    ));

    // ~> 2.4 => >= 2.4, < 2.5
    assert!(check_single_constraint(
        &Version::parse("2.4.0").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2.4").unwrap()
    ));
    assert!(check_single_constraint(
        &Version::parse("2.4.99").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2.4").unwrap()
    ));
    assert!(!check_single_constraint(
        &Version::parse("2.5.0").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2.4").unwrap()
    ));

    // ~> 2.4.1 => >= 2.4.1, < 2.4.2
    assert!(check_single_constraint(
        &Version::parse("2.4.1").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2.4.1").unwrap()
    ));
    assert!(check_single_constraint(
        &Version::parse("2.4.1.9").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2.4.1").unwrap()
    ));
    assert!(!check_single_constraint(
        &Version::parse("2.4.2").unwrap(),
        ConstraintOp::Tilde,
        &Version::parse("2.4.1").unwrap()
    ));
}

// #[test]
fn test_match_constraints_with_special_markers() {
    // beta < release
    assert!(match_constraints(
        &Version::parse("1.0beta").unwrap(),
        &[(ConstraintOp::Lt, Version::parse("1.0").unwrap())]
    ));

    // rc 在 alpha 和 release 之间
    assert!(match_constraints(
        &Version::parse("1.0rc").unwrap(),
        &[
            (ConstraintOp::Ge, Version::parse("1.0alpha").unwrap()),
            (ConstraintOp::Lt, Version::parse("1.0").unwrap()),
        ]
    ));

    // scm > release
    assert!(match_constraints(
        &Version::parse("1.0scm").unwrap(),
        &[(ConstraintOp::Gt, Version::parse("1.0").unwrap())]
    ));
}

// ============= 官方兼容性测试 =============

// #[test]
fn test_official_examples_from_docs() {
    // 来自 luarocks 官方文档和测试的真实例子

    // 1. 基本版本等价
    assert_eq!(compare_version("1.0", "1.0.0").unwrap(), Ordering::Equal);
    assert_eq!(compare_version("1", "1.0.0.0").unwrap(), Ordering::Equal);

    // 2. 预发布版本顺序
    assert!(Version::parse("1.0alpha").unwrap() < Version::parse("1.0beta").unwrap());
    assert!(Version::parse("1.0beta").unwrap() < Version::parse("1.0rc").unwrap());
    assert!(Version::parse("1.0rc").unwrap() < Version::parse("1.0").unwrap());

    // 3. SCM/DEV 版本
    assert!(Version::parse("1.0").unwrap() < Version::parse("1.0scm").unwrap());
    assert!(Version::parse("1.0scm").unwrap() < Version::parse("1.0dev").unwrap());

    // 4. 修订号行为
    assert_eq!(
        Version::parse("1.0")
            .unwrap()
            .cmp(&Version::parse("1.0-0").unwrap()),
        Ordering::Equal
    );
    assert_eq!(
        Version::parse("1.0").unwrap(),
        Version::parse("1.0-0").unwrap()
    );
    assert_eq!(
        Version::parse("1.0-1").unwrap(),
        Version::parse("1.0-1").unwrap()
    );
    assert!(Version::parse("1.0-1").unwrap() < Version::parse("1.0-2").unwrap());

    // 5. 多分隔符等价
    assert_eq!(
        Version::parse("1.2.3").unwrap(),
        Version::parse("1-2_3").unwrap()
    );
}

// #[test]
fn test_edge_cases_from_luarocks_test_suite() {
    // 模拟 luarocks 测试套件中的边界情况

    // 空版本段过滤
    assert_eq!(Version::parse("1..2").unwrap().parts, vec![1, 2]);
    assert_eq!(Version::parse("1--2").unwrap().parts, vec![1]);

    // 大数字（在 i64 范围内）
    assert_eq!(
        Version::parse("999999999").unwrap().parts,
        vec![999_999_999]
    );

    // deltas 产生的负数
    assert_eq!(
        Version::parse("1.0alpha").unwrap().parts,
        vec![1, 0, -1_000_000]
    );
    assert_eq!(
        Version::parse("1.0beta").unwrap().parts,
        vec![1, 0, -100_000]
    );

    // 混合大小写
    assert_eq!(
        Version::parse("1.0BETA").unwrap(),
        Version::parse("1.0beta").unwrap()
    );
    assert_eq!(
        Version::parse("1.0SCM").unwrap(),
        Version::parse("1.0scm").unwrap()
    );

    // 修订号为 0
    assert_eq!(
        Version::parse("1.0").unwrap(),
        Version::parse("1.0-0").unwrap()
    );

    // 只有特殊标记
    let v = Version::parse("scm").unwrap();
    assert_eq!(v.parts, vec![110_000_000]);
}

// #[test]
fn test_sorting_consistency() {
    // 测试排序是否符合预期顺序
    let versions = vec![
        "1.0alpha", "1.0beta", "1.0rc", "1.0pre", "1.0", "1.0-1", "1.0cvs", "1.0scm", "1.0dev",
        "1.0.1", "1.1", "2.0",
    ];

    let mut parsed: Vec<Version> = versions
        .iter()
        .map(|s| Version::parse(s).unwrap())
        .collect();

    parsed.sort();

    // 验证排序后的顺序是非递减的
    for i in 1..parsed.len() {
        assert!(
            parsed[i - 1] <= parsed[i],
            "Sort order violated: {} > {}",
            versions[i - 1],
            versions[i]
        );
    }
}

// #[test]
fn test_from_str_trait() {
    let v: Version = "1.2.3".parse().unwrap();
    assert_eq!(v.parts, vec![1, 2, 3]);

    let v: Version = "1.0-beta-5".parse().unwrap();
    assert_eq!(v.parts, vec![1, 0, -100_000]);
    assert_eq!(v.revision, Some(5));
}

// #[test]
fn test_clone_and_debug() {
    let v1 = Version::parse("1.2.3").unwrap();
    let v2 = v1.clone();
    assert_eq!(v1, v2);

    // 测试 Debug 输出（不崩溃即可）
    let debug = format!("{:?}", v1);
    assert!(debug.contains("Version"));
}

// #[test]
fn test_hash_eq_consistency() {
    // 如果实现 Hash，需要保证 == 的相等性与哈希一致
    // 这里只是验证 PartialEq 行为
    let v1 = Version::parse("1.0").unwrap();
    let v2 = Version::parse("1.0.0").unwrap();
    assert_eq!(v1, v2); // 缺失段视为 0
}

// ============= 压力/边界测试 =============

// #[test]
fn test_long_version_strings() {
    // 长版本字符串
    let long = "1.2.3.4.5.6.7.8.9.10";
    let v = Version::parse(long).unwrap();
    assert_eq!(v.parts.len(), 10);
    assert_eq!(v.parts[9], 10);
}

// #[test]
fn test_zero_values() {
    // 0 作为正常数值
    assert_eq!(Version::parse("0").unwrap().parts, vec![0]);
    assert_eq!(Version::parse("0.0.0").unwrap().parts, vec![0, 0, 0]);
    assert!(Version::parse("0.1").unwrap() > Version::parse("0.0.1").unwrap());
}

// #[test]
fn test_large_revision_numbers() {
    // 大修订号
    let v = Version::parse("1.0-999999").unwrap();
    assert_eq!(v.revision, Some(999_999));

    // 修订号比较
    assert!(Version::parse("1.0-999").unwrap() > Version::parse("1.0-100").unwrap());
}

// #[test]
fn test_mixed_special_and_numbers() {
    // 特殊标记后跟数字：每个作为独立段
    let v = Version::parse("1.0beta3").unwrap();
    assert_eq!(v.parts, vec![1, 0, -100_000, 3]);

    // 比较：1.0beta3 < 1.0（因为 -100000 < 0）
    assert!(v < Version::parse("1.0").unwrap());
}

// #[test]
fn test_constraint_parsing_edge_cases() {
    // 操作符紧贴版本号
    assert_eq!(parse_constraint(">=1.0").unwrap().0, ConstraintOp::Ge);

    // 多余空格
    assert_eq!(
        parse_constraint("  ~>   2.4  ").unwrap(),
        (ConstraintOp::Tilde, "2.4".to_string())
    );

    // 无效操作符（应作为版本名）
    let (op, ver) = parse_constraint("===1.0").unwrap();
    assert_eq!(op, ConstraintOp::Eq); // 默认
    assert!(ver.starts_with("=1.0")); // "=" 作为版本名一部分
}

// #[test]
fn test_match_constraints_empty() {
    // 空约束列表：任何版本都满足
    let v = Version::parse("1.0").unwrap();
    assert!(match_constraints(&v, &[]));
}

// #[test]
fn test_compare_scm_vs_revision() {
    // scm 是数值段，修订号是独立字段
    let scm = Version::parse("1.0scm").unwrap(); // parts=[1,0,110000000], revision=None
    let with_rev = Version::parse("1.0-1").unwrap(); // parts=[1,0], revision=Some(1)

    // 比较 parts: [1,0,110000000] vs [1,0,0] -> scm > with_rev
    assert!(scm > with_rev);
}
