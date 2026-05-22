use std::cmp::Ordering;
use std::str::FromStr;

// -----------------------------------------------------------------------------
// https://github.com/luarocks/luarocks/blob/main/src/luarocks/core/vers.tl
// 特殊标记映射表 (deltas)
// -----------------------------------------------------------------------------
const DELTAS: &[(&str, i64)] = &[
    ("dev", 120_000_000),
    ("scm", 110_000_000),
    ("cvs", 100_000_000),
    ("rc", -1_000),
    ("pre", -10_000),
    ("beta", -100_000),
    ("alpha", -1_000_000),
];

/// 将字符串标记转换为数值（官方逻辑）
/// - 匹配 deltas 表返回对应值
/// - 未知单词：首字符 ASCII 码 / 1000（整数除法）
fn string_to_numeric(s: &str) -> i64 {
    // 1. 查 deltas 表（不区分大小写，官方使用 :lower()）
    for &(key, val) in DELTAS {
        if s.eq_ignore_ascii_case(key) {
            return val;
        }
    }
    // 2. 未知单词：首字符 ASCII / 1000
    s.chars().next().map(|c| c as i64 / 1000).unwrap_or(0)
}

// -----------------------------------------------------------------------------
// 版本核心结构
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Eq)]
pub struct Version {
    pub parts: Vec<i64>,       // 版本段列表（全部为数值，支持负数和大数）
    pub revision: Option<u32>, // 修订号（末尾的-数字），None 表示无修订号
}

// 实现 PartialEq 以支持版本比较符 ==
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl Version {
    /// 解析版本字符串（严格兼容官方 vers.tl）
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty version string".to_string());
        }

        // 1. 提取修订号（官方逻辑：贪婪匹配末尾的 -数字）
        // 注意：这是贪婪匹配，会匹配到最后一个 -数字 组合
        let (main, revision) = if let Some(pos) = s.rfind('-') {
            let after = &s[pos + 1..];
            // 条件：after 非空 + 全数字 + pos>0 确保 before 非空
            if !after.is_empty() && after.chars().all(|c| c.is_ascii_digit()) && pos > 0 {
                let rev = after
                    .parse::<u32>()
                    .map_err(|e| format!("Invalid revision: {}", e))?;
                (&s[..pos], Some(rev))
            } else {
                (s, None)
            }
        } else {
            (s, None)
        };

        // 2. 词法分析：逐个提取数字或字母单词（官方核心逻辑！）
        let parts = parse_tokens(main);

        Ok(Version { parts, revision })
    }

    /// 版本比较（缺失段视为0，修订号仅双方有时比较）
    pub fn compare(&self, other: &Self) -> Ordering {
        let max_len = self.parts.len().max(other.parts.len());
        for i in 0..max_len {
            let a = self.parts.get(i).copied().unwrap_or(0);
            let b = other.parts.get(i).copied().unwrap_or(0);
            match a.cmp(&b) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }
        match (self.revision, other.revision) {
            (Some(a), Some(b)) => a.cmp(&b),
            _ => Ordering::Equal,
        }
    }
}

/// 词法分析器，兼容官方 vers.tl 的 token 提取逻辑
///
/// 官方规则：
/// 1. 优先匹配连续数字 \d+
/// 2. 再匹配连续字母 \a+
/// 3. 分隔符 [.%-_] 是可选的，用于分隔但不参与分割
/// 4. 其他字符跳过
fn parse_tokens(s: &str) -> Vec<i64> {
    let mut parts = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(&ch) = chars.peek() {
        // 跳过分隔符 [.%-_]
        if ch == '.' || ch == '-' || ch == '_' {
            chars.next();
            continue;
        }

        // 匹配连续数字
        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            if let Ok(num) = num_str.parse::<i64>() {
                parts.push(num);
            }
            continue;
        }

        // 匹配连续字母（大小写）
        if ch.is_ascii_alphabetic() {
            let mut word = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_alphabetic() {
                    word.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            parts.push(string_to_numeric(&word));
            continue;
        }

        // 其他字符：跳过（官方行为：忽略非字母数字字符）
        chars.next();
    }

    parts
}

// 实现标准 trait 以便直接使用比较运算符
impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl FromStr for Version {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

// -----------------------------------------------------------------------------
// 约束操作符（支持官方所有操作符）
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintOp {
    Eq,    // ==
    Ne,    // ~= (不等于)
    Lt,    // <
    Le,    // <=
    Gt,    // >
    Ge,    // >=
    Tilde, // ~> (pessimistic operator)
}

/// 解析约束字符串，如 ">= 1.0"、"~> 2.4"
pub fn parse_constraint(s: &str) -> Result<(ConstraintOp, String), String> {
    let s = s.trim();

    // 注意顺序：先检查双字符操作符，避免单字符误匹配
    if let Some(v) = s.strip_prefix("~>") {
        return Ok((ConstraintOp::Tilde, v.trim().to_string()));
    }
    if let Some(v) = s.strip_prefix("==") {
        return Ok((ConstraintOp::Eq, v.trim().to_string()));
    }
    if let Some(v) = s.strip_prefix("~=") {
        return Ok((ConstraintOp::Ne, v.trim().to_string()));
    }
    if let Some(v) = s.strip_prefix("<=") {
        return Ok((ConstraintOp::Le, v.trim().to_string()));
    }
    if let Some(v) = s.strip_prefix(">=") {
        return Ok((ConstraintOp::Ge, v.trim().to_string()));
    }
    if let Some(v) = s.strip_prefix('<') {
        return Ok((ConstraintOp::Lt, v.trim().to_string()));
    }
    if let Some(v) = s.strip_prefix('>') {
        return Ok((ConstraintOp::Gt, v.trim().to_string()));
    }

    // 没有操作符时默认为精确相等（官方行为）
    Ok((ConstraintOp::Eq, s.to_string()))
}

/// 检查版本是否满足单个约束
pub fn check_single_constraint(version: &Version, op: ConstraintOp, target: &Version) -> bool {
    match op {
        ConstraintOp::Eq => version == target,
        ConstraintOp::Ne => version != target,
        ConstraintOp::Lt => version < target,
        ConstraintOp::Le => version <= target,
        ConstraintOp::Gt => version > target,
        ConstraintOp::Ge => version >= target,
        ConstraintOp::Tilde => {
            // ~> 悲观版本约束（官方 partial_match 逻辑）
            // ~> 2      => >= 2, < 3
            // ~> 2.4    => >= 2.4, < 2.5
            // ~> 2.4.1  => >= 2.4.1, < 2.4.2
            if version < target {
                return false;
            }

            // 计算上限：最后一个段 +1，后面截断
            let mut upper_parts = target.parts.clone();
            if let Some(last) = upper_parts.last_mut() {
                *last += 1;
            } else {
                upper_parts.push(1);
            }
            let upper = Version {
                parts: upper_parts,
                revision: None, // 上限不考虑修订号
            };

            version < &upper
        }
    }
}

/// 检查版本是否满足所有约束（与 `vers.match_constraints` 行为一致）
pub fn match_constraints(version: &Version, constraints: &[(ConstraintOp, Version)]) -> bool {
    constraints
        .iter()
        .all(|(op, target)| check_single_constraint(version, *op, target))
}

pub fn compare_version(a: &str, b: &str) -> Result<Ordering, String> {
    let version_a = Version::parse(a)?;
    let version_b = Version::parse(b)?;

    match version_a.cmp(&version_b) {
        Ordering::Equal => Ok(Ordering::Equal),
        Ordering::Less => Ok(Ordering::Less),
        Ordering::Greater => Ok(Ordering::Greater),
    }
}
