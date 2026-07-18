//! ## 最长公共前缀
//!
//! 给出一组字符串，找出这些字符串公共最长前缀。
//! 如果不存在，返回空串。
//!
//! 取值范围：
//! - 1 <= strs.length <= 200
//! - 0 <= strs[i].length <= 200
//! - strs[i] 如果非空，则仅由小写英文字母组成

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        assert!(strs.iter().all(|x| x.is_ascii()), "Invalid input");

        if strs.len() == 1 {
            return strs[0].clone();
        }

        let rng = strs
            .iter()
            .map(|x| x.len())
            .min()
            .expect("Input str number is zero, invlaid");

        let mut i: usize = 0;
        while i <= rng {
            let same = strs
                .iter()
                .map(|x| &x[0..i])
                .collect::<Vec<&str>>()
                .windows(2)
                .all(|w| w[0] == w[1]);

            if !same {
                return strs[0].clone()[0..(i - 1)].to_string();
            }

            i += 1;
        }

        strs[0].clone()[0..rng].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        strs: Vec<String>,
        answer: String,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                strs: vec![
                    "flower".to_string(),
                    "flow".to_string(),
                    "flight".to_string(),
                ],
                answer: "fl".to_string(),
            },
            TestCase {
                strs: vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
                answer: "".to_string(),
            },
            TestCase {
                strs: vec!["ab".to_string(), "a".to_string()],
                answer: "a".to_string(),
            },
            TestCase {
                strs: vec!["a".to_string(), "b".to_string()],
                answer: "".to_string(),
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::longest_common_prefix(c.strs);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
