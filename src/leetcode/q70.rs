//! ## 爬楼梯
//!
//! 爬楼梯，有 n 阶才能到楼顶。
//! 每次可以爬 1 阶或者 2 阶。
//! 输出有多少种不同的方法可以爬到楼顶。
//!
//! **限制条件：**
//! - $1 <= n <= 45$

pub struct Solution;

impl Solution {
    /// n 阶楼梯，最后爬上去时的最后一步可能爬 2 阶，也可能爬 1 阶
    /// 设 f(n) 为爬上 n 阶楼梯的不同方案数，则有：
    /// $$
    /// f(n) = f(n - 1) + f(n - 2), n >= 2
    /// $$
    ///
    /// 使用递归：
    ///
    /// ```
    /// pub fn climb_stairs(n: i32) -> i32 {
    ///     if n == 1 {
    ///         return 1;
    ///     }
    ///     if n == 2 {
    ///         return 2;
    ///     }
    ///     climb_stairs(n - 2) + climb_stairs(n - 1)
    /// }
    /// ```
    ///
    /// 使用递归会有较高的复杂度，所以使用迭代解法。
    pub fn climb_stairs(n: i32) -> i32 {
        let mut f = Vec::with_capacity(n as usize);
        f.push(1);
        f.push(2);

        for i in 2..(n as usize) {
            f.push(f[i - 1] + f[i - 2]);
        }

        f[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        n: i32,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase { n: 2, answer: 2 },
            TestCase { n: 3, answer: 3 },
            TestCase { n: 4, answer: 5 },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::climb_stairs(c.n);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
