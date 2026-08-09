//! ## 杨辉三角 2
//!
//! 给定一个非负整索引 `rowIndex`，返回杨辉三角的第 `rowIndex` 行。
//! 在杨辉三角中，每个数是它左上角和右上角的数的和。
//!
//! **限制条件：**
//! - `1 <= rowIndex <= 33`

pub struct Solution;

impl Solution {
    /// 空间复杂度 O(rowIndex)
    pub fn get_row(row_index: i32) -> Vec<i32> {
        assert!(row_index >= 0);

        let mut result = vec![1];
        for _i in 0..row_index {
            result.push(1);
            for j in (1..(result.len() - 1)).rev() {
                result[j] += result[j - 1];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        row_index: i32,
        answer: Vec<i32>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                row_index: 3,
                answer: vec![1, 3, 3, 1],
            },
            TestCase {
                row_index: 0,
                answer: vec![1],
            },
            TestCase {
                row_index: 1,
                answer: vec![1, 1],
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::get_row(c.row_index);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
