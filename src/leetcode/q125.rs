//! ## 验证回文串
//!
//! 如果在将所有大写字符转换为小写字符、并移除所有非字母数字字符之后，短语正着读和反着读都一样。则可以认为该短语是一个 回文串 。
//! 字母和数字都属于字母数字字符。
//! 给你一个字符串 s，如果它是 回文串 ，返回 true ；否则，返回 false 。
//!
//! **限制条件：**
//! - $1 \le s.length \le 2 * 10^5$
//! - `s` 仅由可打印的 ASCII 字符组成

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let ss = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();

        if ss.len() < 2 {
            return true;
        }

        let mut i = 0;
        let mut j = ss.len() - 1;
        while i < j {
            if ss.chars().nth(i) != ss.chars().nth(j) {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        s: String,
        answer: bool,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                s: "A man, a plan, a canal: Panama".to_string(),
                answer: true,
            },
            TestCase {
                s: "A man".to_string(),
                answer: false,
            },
            TestCase {
                s: " ".to_string(),
                answer: true,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::is_palindrome(c.s);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
