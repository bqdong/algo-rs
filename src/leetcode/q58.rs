//! ## 最后一个单词的长度
//!
//! 给一个字符串 s，有若干字母和空格组成，返回最后一个单词长度。
//!
//! **限制条件：**
//! - $1 <= s <= 10^4$
//! - `s` 仅有英文字母和空格组成
//! - `s` 中至少存在一个单词

pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut p1 = s.len() - 1;
        let mut p2 = p1;
        loop {
            if s.chars().nth(p2) == Some(' ') {
                p1 -= 1;
                p2 -= 1;
                continue;
            }
            if p1 == 0 || s.chars().nth(p1) == Some(' ') {
                break;
            }
            p1 -= 1;
        }
        if s.chars().nth(p1) == Some(' ') {
            (p2 - p1) as i32
        } else {
            (p2 - p1 + 1) as i32
        }
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
                s: "Hello World".to_string(),
                answer: 5,
            },
            TestCase {
                s: "   fly me   to   the moon  ".to_string(),
                answer: 4,
            },
            TestCase {
                s: "luffy is still joyboy".to_string(),
                answer: 6,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::length_of_last_word(c.s);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
