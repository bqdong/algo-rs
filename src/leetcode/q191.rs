//! ## [位 1 的个数](https://leetcode.cn/problems/number-of-1-bits/description/)
//!
//! 给定一个正整数 n，编写一个函数，获取一个正整数的二进制形式并返回其二进制表达式中 设置位(位为1) 的个数（也被称为汉明重量）。
//!
//! **限制条件：**
//! - $1 \le n \le 2^31 - 1$

pub struct Solution;

impl Solution {
    fn repr_binary(mut n: i32) -> [u8; 32] {
        let mut repr = [0; 32];
        let mut idx = 31;
        while n > 0 {
            repr[idx] = (n % 2) as u8;
            n /= 2;
            idx -= 1;
        }
        repr
    }

    /// TODO: 多次调用如何优化？
    pub fn hamming_weight(n: i32) -> i32 {
        Solution::repr_binary(n)
            .into_iter()
            .filter(|e| e == &1)
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        n: i32,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase { n: 11, answer: 3 },
            TestCase { n: 128, answer: 1 },
            TestCase {
                n: 2147483645,
                answer: 30,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |(idx, c): (usize, TestCase)| {
            let result = Solution::hamming_weight(c.n);
            assert_eq!(c.answer, result, "The {} test fails", idx);
        };
        cases.into_iter().enumerate().for_each(t);
    }
}
