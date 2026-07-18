//! ## 有效的括号
//!
//! 给定一个只包含 `()[]{}` 等 6 种字符的字符串，判断是否合法：
//! - 左括号必须使用相同的右括号闭合
//! - 左括号必须以正确顺序闭合
//! - 每个右括号右一个对应类型的左括号
//!
//! 取值范围：
//! - 1 <= s.length <= 10^4

pub struct Solution;

impl Solution {
    /// 使用栈来解或者从两头向中间方向遍历判断是否匹配
    pub fn is_valid(s: String) -> bool {
        assert!(
            s.chars().all(|c| "()[]{}".contains(c)),
            "Invalid input string"
        );

        if !s.len().is_multiple_of(2) {
            return false;
        }

        let mut stack = vec![];
        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();
            if "([{".contains(c) {
                stack.push(c);
            } else {
                let should = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    _ => panic!("Illegal state"),
                };
                let prev = stack.pop();
                if prev != Some(should) {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        s: String,
        answer: bool,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                s: "()".to_string(),
                answer: true,
            },
            TestCase {
                s: "()[]{}".to_string(),
                answer: true,
            },
            TestCase {
                s: "(]".to_string(),
                answer: false,
            },
            TestCase {
                s: "([])".to_string(),
                answer: true,
            },
            TestCase {
                s: "([)]".to_string(),
                answer: false,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::is_valid(c.s);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
