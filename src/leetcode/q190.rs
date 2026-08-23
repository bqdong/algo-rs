//! ## [颠倒二进制位](https://leetcode.cn/problems/reverse-bits/description/)
//!
//! 颠倒给定的 32 位有符号整数的二进制位。输出颠倒后的数的十进制。
//!
//! **限制条件：**
//! - 输入的数为偶数
//! - $0 \le n \le 2^31 - 2$

pub struct Solution;

impl Solution {
    const fn bits_num() -> [i32; 32] {
        let mut result = [0; 32];
        let mut i = 0;
        while i < 31 {
            result[31 - i] = 1 << i;
            i += 1;
        }
        result
    }

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

    /// `n` 为偶数说明二进制的最后一位是 0，则表示颠倒后的数肯定位正数。
    /// Rust 提供了 [`i32::reverse_bits`](https://doc.rust-lang.org/std/primitive.i32.html#method.reverse_bits) 方法，其内部调用：
    /// [intrinsics::bitreverse](https://doc.rust-lang.org/src/core/num/uint_macros.rs.html#760) 方法，是一个编译器内部方法
    pub fn reverse_bits(n: i32) -> i32 {
        const BITS: [i32; 32] = Solution::bits_num();
        let bin = Solution::repr_binary(n);
        bin.into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, e| acc + BITS[e.0] * e.1 as i32)
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
            TestCase {
                n: 43261596,
                answer: 964176192,
            },
            TestCase {
                n: 2147483644,
                answer: 1073741822,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |(idx, c): (usize, TestCase)| {
            let result = Solution::reverse_bits(c.n);
            assert_eq!(c.answer, result, "The {} test fails", idx);
        };
        cases.into_iter().enumerate().for_each(t);
    }
}
