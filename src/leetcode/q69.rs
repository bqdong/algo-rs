//! ## x 的平方根
//!
//! 给一个非负整数 x，计算并返回其算术平方根。
//! 返回类型是整数，结果只保留整数部分，舍弃小数部分。
//!
//! 限制条件：
//! - 0 <= x <= 2^31 - 1

pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        if x <= 3 {
            return 1;
        }

        let mut result = 2;
        while result <= x / 2 {
            let next = result + 1;
            let result_div = x / result;
            let next_div = x / next;
            if result_div >= result && next_div < next {
                return result;
            }

            result = next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        x: i32,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase { x: 4, answer: 2 },
            TestCase { x: 5, answer: 2 },
            TestCase { x: 6, answer: 2 },
            TestCase { x: 50, answer: 7 },
            TestCase { x: 100, answer: 10 },
            TestCase { x: 101, answer: 10 },
            TestCase { x: 99, answer: 9 },
            // 要考虑可能有溢出，数据的取值范围考虑清楚
            TestCase {
                x: 2147395600,
                answer: 46340,
            },
            TestCase {
                x: 8,
                answer: 2, // 8 的算术平方根大于 2 小于 3 舍弃小数位则为 2
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::my_sqrt(c.x);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
