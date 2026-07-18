//! ## 回文数
//!
//! 给定一个整数，如果是回文数，返回 true，否则 false。
//!
//! 回文数指的是从左至右或从右至左读，都是同一个数。
//!
//! 取值范围：
//! - -2^{-31} <= x <= 2^31 - 1

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let xcs: Vec<_> = x.to_string().chars().collect();
        for i in 0..(xcs.len() / 2) {
            if xcs[i] != xcs[xcs.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        x: i32,
        answer: bool,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                x: 121,
                answer: true,
            },
            TestCase {
                x: -121,
                answer: false,
            },
            TestCase {
                x: 10,
                answer: false,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::is_palindrome(c.x);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
