//! ## 加一
//!
//! 一个整数，其中各个位的数字从左至右存在数组中（索引 0 是最高位的数字）。
//! 输出一个数组存储该整数加一的结果的各个位的数字。
//!
//! 限制条件：
//! - 1 <= digits.length <= 100
//! - 0 <= digits[i] <= 9
//! - digitals 不包含任何前导 0

pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let remain = (digits[digits.len() - 1] + 1) % 10;
        let mut left = remain == 0;
        result.push(remain);

        for i in (0..(digits.len() - 1)).rev() {
            let cur_digit = digits[i] + i32::from(left);
            if cur_digit == 10 {
                left = true;
                result.push(0);
            } else {
                left = false;
                result.push(cur_digit);
            }
        }

        if left {
            result.push(1);
        }

        let mut start = 0;
        let mut end = result.len() - 1;
        while start < end {
            result.swap(start, end);
            start += 1;
            end -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        digits: Vec<i32>,
        answer: Vec<i32>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                digits: vec![1, 0, 0, 0, 0],
                answer: vec![1, 0, 0, 0, 1],
            },
            TestCase {
                digits: vec![1, 2, 3],
                answer: vec![1, 2, 4],
            },
            TestCase {
                digits: vec![4, 3, 2, 1],
                answer: vec![4, 3, 2, 2],
            },
            TestCase {
                digits: vec![9],
                answer: vec![1, 0],
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::plus_one(c.digits);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
