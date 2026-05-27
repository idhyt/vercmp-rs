use std::cmp::Ordering;

fn parse_version(s: &str) -> Result<Vec<u32>, String> {
    s.split(|c: char| c == '.' || c == '-')
        .map(|seg| {
            seg.parse::<u32>()
                .map_err(|_| format!("invalid segment: {}", seg))
        })
        .collect()
}

/// 比较两个版本号字符串，支持 NA（None）语义。
/// https://github.com/wch/r-source/blob/trunk/src/library/utils/R/packages.R#L1183
/// 对应 R 函数 `compareVersion` 的逻辑：
/// - 若 a 为 None（相当于 NA），返回 Ordering::Less（无论 b 是什么）。
/// - 若 a 非 None 但 b 为 None，返回 Ordering::Greater。
/// - 否则，解析版本号并逐位比较：
///   * 从左到右比较整数段，较大的版本号胜出。
///   * 如果前缀相同但 a 的段数更多，a 胜出。
///   * 如果前缀相同但 b 的段数更多，b 胜出。
///   * 完全相同则返回 Ordering::Equal。
fn compare_r_version(a: Option<&str>, b: Option<&str>) -> Result<Ordering, String> {
    // 处理 NA 情况，完全模拟 R 逻辑
    if a.is_none() {
        return Ok(Ordering::Less); // is.na(a) → -1
    }
    if b.is_none() {
        return Ok(Ordering::Greater); // is.na(b) → 1 (a 已确认非 NA)
    }

    let a_parts = parse_version(a.unwrap())?;
    let b_parts = parse_version(b.unwrap())?;

    for (k, &a_val) in a_parts.iter().enumerate() {
        if k < b_parts.len() {
            let b_val = b_parts[k];
            if a_val > b_val {
                return Ok(Ordering::Greater);
            } else if a_val < b_val {
                return Ok(Ordering::Less);
            }
        } else {
            // a 比 b 长，且前缀全相等，a 更大
            return Ok(Ordering::Greater);
        }
    }

    // 循环结束说明 a 的所有段都与 b 的前缀相等
    if b_parts.len() > a_parts.len() {
        Ok(Ordering::Less) // b 更长，b 更大
    } else {
        Ok(Ordering::Equal) // 完全相等
    }
}

pub fn compare_version(a: &str, b: &str) -> Result<Ordering, String> {
    compare_r_version(
        if a.is_empty() { None } else { Some(a) },
        if b.is_empty() { None } else { Some(b) },
    )
}
