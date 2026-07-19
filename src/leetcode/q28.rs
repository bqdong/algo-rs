//! ## 找出字符串中第一个匹配的下标（子串）
//!
//! 给定一个字符串 `haystack` 和 `needle`，在 `haystack` 中找出 `needle`
//! 第一次出现的下标，找不到返回 -1 。
//!
//! 取值范围：
//! - 1 <= haystack.length, needle.length <= 10^4
//! - 两个字符串都由小写英文字符组成

pub struct Solution;

impl Solution {
    /// 找子串的算法，这里直接用暴力解法
    /// TODO: KMP
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }

        'outer: for i in 0..=(haystack.len() - needle.len()) {
            for j in 0..needle.len() {
                if haystack.chars().nth(i + j) != needle.chars().nth(j) {
                    continue 'outer;
                }
            }
            return i.try_into().expect("Invalid state");
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        haystack: String,
        needle: String,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                haystack: "sadbutsad".to_string(),
                needle: "sad".to_string(),
                answer: 0,
            },
            TestCase {
                haystack: "leetcode".to_string(),
                needle: "leeto".to_string(),
                answer: -1,
            },
            TestCase {
                haystack: "a".to_string(),
                needle: "a".to_string(),
                answer: 0,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::str_str(c.haystack, c.needle);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
