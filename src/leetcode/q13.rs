//! ## 罗马数字转整数
//!
//! 罗马字母代表的数字大小：
//! - I -> 1
//! - V -> 5
//! - X -> 10
//! - L -> 50
//! - C -> 100
//! - D -> 500
//! - M -> 1000
//!
//! 通常情况，表示大数的字母在左，小的在右侧，如 `VI` 表示 6，
//! `XII` 表示 12。
//!
//! 特殊情况：
//! - 4 写作 `IV`，小的在左，大的在右，表示 `5 -1 = 4`
//! - 9 表示为 `IX`
//!
//! 特殊规则只适用于以下 6 种情况：
//! - `I` 可以放在 `V` 和 `X` 的左边，表示 4 和 9
//! - `X` 可以放在 `L` 和 `C` 的左边，表示 40 和 90
//! - `C` 可以放在 `D` 和 `M` 的左边，表示 400 和 900
//!
//! 取值范围：
//! - -2^{-31} <= x <= 2^31 - 1

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if !s.is_ascii() {
            // prevent panic at following string slice operation
            panic!("Illegal input string");
        }

        let mut result = 0;

        let mut i = 0;
        while i < s.len() {
            if i < s.len() - 1 {
                let cs = &s[i..=i + 1];
                let matched = match cs {
                    "IV" => 4,
                    "IX" => 9,
                    "XL" => 40,
                    "XC" => 90,
                    "CD" => 400,
                    "CM" => 900,
                    _ => 0,
                };

                if matched > 0 {
                    result += matched;
                    i += 2;
                    continue;
                }
            }

            result += match &s[i..i + 1] {
                "I" => 1,
                "V" => 5,
                "X" => 10,
                "L" => 50,
                "C" => 100,
                "D" => 500,
                "M" => 1000,
                _ => panic!("Invalid chars in input string"),
            };

            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        s: String,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                s: "III".to_string(),
                answer: 3,
            },
            TestCase {
                s: "IV".to_string(),
                answer: 4,
            },
            TestCase {
                s: "LVIII".to_string(),
                answer: 58,
            },
            TestCase {
                s: "MCMXCIV".to_string(),
                answer: 1994,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::roman_to_int(c.s);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
