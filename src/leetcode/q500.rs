//! ## [键盘行](https://leetcode.cn/problems/keyboard-row/description/)
//!
//! 给你一个字符串数组 words ，只返回可以使用在 美式键盘 同一行的字母打印出来的单词。
//! 请注意，字符串 不区分大小写，相同字母的大小写形式都被视为在同一行。
//!
//! **美式键盘**中：
//! - 第一行由字符 `qwertyuiop` 组成
//! - 第二行由字符 `asdfghjkl` 组成
//! - 第三行由字符 `zxcvbnm` 组成
//!
//! **限制条件：**
//! - $1 \le n \le 2^31 - 1$

pub struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let written_in_same_line = |e: &str| {
            let lines = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
            for l in lines {
                let lo = e.to_lowercase();
                let not_contains = lo.chars().filter(|c| !l.contains(*c)).count();
                if not_contains == 0 {
                    return true;
                }
            }
            return false;
        };

        words
            .iter()
            .filter(|word| written_in_same_line(word))
            .map(|s| s.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        words: Vec<String>,
        answer: Vec<String>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                words: vec![
                    "Hello".to_owned(),
                    "Alaska".to_owned(),
                    "Dad".to_owned(),
                    "Peace".to_owned(),
                ],
                answer: vec!["Alaska".to_owned(), "Dad".to_owned()],
            },
            TestCase {
                words: vec!["omk".to_owned()],
                answer: vec![],
            },
            TestCase {
                words: vec!["adsdf".to_owned(), "sfd".to_owned()],
                answer: vec!["adsdf".to_owned(), "sfd".to_owned()],
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |(idx, c): (usize, TestCase)| {
            let result = Solution::find_words(c.words);
            assert_eq!(c.answer, result, "The {} test fails", idx);
        };
        cases.into_iter().enumerate().for_each(t);
    }
}
