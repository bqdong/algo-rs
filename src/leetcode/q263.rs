//! ## 丑数
//!
//! 丑数是指只包含质数因数 2、3、5 的正整数。
//!
//! 限制条件：
//! - -2^31 <= n <= 2^31 - 1

pub struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        if n <= 6 {
            return true;
        }
        for i in 2..=6 {
            if n % i == 0 {
                return Solution::is_ugly(n / i);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        n: i32,
        answer: bool,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase { n: 6, answer: true },
            TestCase { n: 1, answer: true },
            TestCase {
                n: 14,
                answer: false,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::is_ugly(c.n);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
