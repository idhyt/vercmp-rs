//! # opam 风格的版本比较（基于 Debian 策略）
//!
//! 此模块 1:1 地复刻了 `https://github.com/ocaml/opam/blob/master/src/core/opamVersionCompare.ml` 中的算法。
//! 该算法本身是 `dose3` 库中 `debian.ml` 的变体，完全遵循
//! Debian 策略手册 (3.9.2/3.4.6) 中描述的版本字符串比较规则。
//!
//! **重要**：该实现 **不处理 Debian epoch**（如 `1:2.0`），
//! 冒号仅视为普通字符。这是 opam 原始实现的行为，因为 opam 包
//! 的版本字段从不包含 epoch。
//!
//! 比较流程：
//! 1. 按最后一个 `-` 分割出上游版本和 Debian 修订版本。
//! 2. 比较上游部分。
//! 3. 若上游部分相等，则比较修订部分。
//! 4. 每个部分都以词法模式开始，在数字和非数字片段之间交替，
//!    并对数字片段使用数值比较。

use std::cmp::Ordering;

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn skip_while_from(i: usize, f: fn(u8) -> bool, w: &[u8], m: usize) -> usize {
    let mut cur = i;
    while cur < m && f(w[cur]) {
        cur += 1;
    }
    cur
}

fn compare_chars(c1: u8, c2: u8) -> i32 {
    match c1 {
        b'~' => match c2 {
            b'~' => 0,
            _ => -1,
        },
        b'a'..=b'z' | b'A'..=b'Z' => match c2 {
            b'~' => 1,
            b'a'..=b'z' | b'A'..=b'Z' => c1.cmp(&c2) as i32,
            _ => -1,
        },
        _ => match c2 {
            b'~' | b'a'..=b'z' | b'A'..=b'Z' => 1,
            _ => c1.cmp(&c2) as i32,
        },
    }
}

fn skip_zeros(x: &[u8], xi: usize, xl: usize) -> usize {
    skip_while_from(xi, |c| c == b'0', x, xl)
}

fn loop_lexical(xl: usize, yl: usize, x: &[u8], y: &[u8], xi: usize, yi: usize) -> i32 {
    match (xi == xl, yi == yl) {
        (true, true) => 0,
        (true, false) => {
            let ys = skip_zeros(y, yi, yl);
            if ys == yl {
                0
            } else if y[ys] == b'~' {
                1
            } else {
                -1
            }
        }
        (false, true) => {
            let xs = skip_zeros(x, xi, xl);
            if xs == xl {
                0
            } else if x[xs] == b'~' {
                -1
            } else {
                1
            }
        }
        (false, false) => match (is_digit(x[xi]), is_digit(y[yi])) {
            (true, true) => {
                compare_numerical(xl, yl, x, y, skip_zeros(x, xi, xl), skip_zeros(y, yi, yl))
            }
            (true, false) => {
                if y[yi] == b'~' {
                    1
                } else {
                    -1
                }
            }
            (false, true) => {
                if x[xi] == b'~' {
                    -1
                } else {
                    1
                }
            }
            (false, false) => {
                let comp = compare_chars(x[xi], y[yi]);
                if comp == 0 {
                    loop_lexical(xl, yl, x, y, xi + 1, yi + 1)
                } else {
                    comp
                }
            }
        },
    }
}

fn compare_numerical(xl: usize, yl: usize, x: &[u8], y: &[u8], xi: usize, yi: usize) -> i32 {
    let xn = skip_while_from(xi, is_digit, x, xl);
    let yn = skip_while_from(yi, is_digit, y, yl);
    let comp = (xn - xi).cmp(&(yn - yi)) as i32;
    if comp != 0 {
        return comp;
    }
    loop_numerical(xl, yl, x, y, xi, yi, yn)
}

fn loop_numerical(
    xl: usize,
    yl: usize,
    x: &[u8],
    y: &[u8],
    xi: usize,
    yi: usize,
    yn: usize,
) -> i32 {
    if yi == yn {
        return loop_lexical(xl, yl, x, y, xi, yi);
    }
    let comp = x[xi].cmp(&y[yi]) as i32;
    if comp == 0 {
        loop_numerical(xl, yl, x, y, xi + 1, yi + 1, yn)
    } else {
        comp
    }
}

fn normalize_comp_result(x: i32) -> i32 {
    match x {
        0 => 0,
        _ if x < 0 => -1,
        _ => 1,
    }
}

pub fn compare(x: &str, y: &str) -> Ordering {
    if x == y {
        return Ordering::Equal;
    }
    let xb = x.as_bytes();
    let yb = y.as_bytes();
    let lx = xb.len();
    let ly = yb.len();

    let rx = xb.iter().rposition(|&c| c == b'-').unwrap_or(lx);
    let ry = yb.iter().rposition(|&c| c == b'-').unwrap_or(ly);

    let u_comp = loop_lexical(rx, ry, xb, yb, 0, 0);
    if u_comp != 0 {
        return match normalize_comp_result(u_comp) {
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => unreachable!(),
        };
    }

    let r_comp = loop_lexical(lx, ly, xb, yb, (rx + 1).min(lx), (ry + 1).min(ly));
    match normalize_comp_result(r_comp) {
        -1 => Ordering::Less,
        1 => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

pub fn equal(x: &str, y: &str) -> bool {
    x == y || compare(x, y) == Ordering::Equal
}

pub fn compare_version(a: &str, b: &str) -> Ordering {
    compare(a, b)
}
