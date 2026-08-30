//! ## [快乐数](https://leetcode.cn/problems/happy-number/description/)
//!
//! 编写一个算法来判断一个数 n 是不是快乐数。
//! 快乐数定义为：
//! - 对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
//! - 然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
//! - 如果这个过程 结果为 1，那么这个数就是快乐数。
//! - 如果 n 是 快乐数 就返回 true ；不是，则返回 false
//!
//! **限制条件：**
//! - $1 \le n \le 2^31 - 1$

pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut trace = vec![];
        let split_n = |mut e: i32| {
            let mut r = vec![];
            while e > 0 {
                r.push(e % 10);
                e /= 10;
            }
            r
        };

        let mut x = n;
        loop {
            if trace.contains(&x) {
                return false;
            }

            let sum = split_n(x).iter().map(|e| e.pow(2)).sum();
            if sum == 1 {
                return true;
            }

            trace.push(x);
            x = sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        n: i32,
        answer: bool,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                n: 19,
                answer: true,
            },
            TestCase {
                n: 2,
                answer: false,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |(idx, c): (usize, TestCase)| {
            let result = Solution::is_happy(c.n);
            assert_eq!(c.answer, result, "The {} test fails", idx);
        };
        cases.into_iter().enumerate().for_each(t);
    }
}
