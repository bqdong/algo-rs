//! ## 二进制求和
//!
//! 给两个二进制字符串 a 和 b，返回二进制求和结果
//!
//! **限制条件：**
//! - $1 <= a.length, b.length <= 10^4$
//! - a, b 仅包含字符 0 和 1
//! - 字符串如果不是 0 ，就不含前导 0

pub struct Solution;

impl Solution {
    fn into_rev_pad_iter(s: &str, pad_len: usize) -> impl Iterator<Item = char> {
        s.chars()
            .rev()
            .chain(std::iter::repeat_n('0', pad_len.saturating_sub(s.len())))
    }

    pub fn add_binary(a: String, b: String) -> String {
        let max_len = if a.len() > b.len() { a.len() } else { b.len() };

        let mut carry = false;
        let mut rev_result = std::iter::zip(
            Solution::into_rev_pad_iter(&a, max_len),
            Solution::into_rev_pad_iter(&b, max_len),
        )
        .fold(String::new(), |mut acc, e| {
            let (digit, car) = match (e, carry) {
                (('0', '0'), false) => ('0', false),
                (('0', '0'), true) => ('1', false),
                (('0', '1'), false) | (('1', '0'), false) => ('1', false),
                (('0', '1'), true) | (('1', '0'), true) => ('0', true),
                (('1', '1'), false) => ('0', true),
                (('1', '1'), true) => ('1', true),
                _ => unreachable!(),
            };
            carry = car;

            acc.push(digit);
            acc
        });

        if carry {
            rev_result.push('1');
        }

        rev_result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        a: String,
        b: String,
        answer: String,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                a: "11".to_string(),
                b: "1".to_string(),
                answer: "100".to_string(),
            },
            TestCase {
                a: "1010".to_string(),
                b: "1011".to_string(),
                answer: "10101".to_string(),
            },
            TestCase {
                a: "0".to_string(),
                b: "0".to_string(),
                answer: "0".to_string(),
            },
            TestCase {
                a: "1".to_string(),
                b: "0".to_string(),
                answer: "1".to_string(),
            },
            TestCase {
                a: "1".to_string(),
                b: "1".to_string(),
                answer: "10".to_string(),
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::add_binary(c.a, c.b);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
