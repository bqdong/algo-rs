//! ## 杨辉三角
//!
//! 给定一个非负整数 `numRows`，生成杨辉三角的前 `numRows` 行。
//! 在杨辉三角中，每个数是它左上角和右上角的数的和。
//!
//! **限制条件：**
//! - `1 <= numRows <= 30`

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        assert!(num_rows >= 0);

        if num_rows == 0 {
            return vec![];
        }

        let mut result = vec![vec![1]];
        if num_rows == 1 {
            return result;
        }

        for row in 2..=(num_rows as usize) {
            let mut cur_row: Vec<i32> = vec![];
            let last_row = &result[row - 2];
            for i in 0..row {
                let n = if i >= 1 && i < row - 1 {
                    last_row[i - 1] + last_row[i]
                } else {
                    1
                };
                cur_row.push(n);
            }

            result.push(cur_row);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        num_rows: i32,
        answer: Vec<Vec<i32>>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                num_rows: 1,
                answer: vec![vec![1]],
            },
            TestCase {
                num_rows: 2,
                answer: vec![vec![1], vec![1, 1]],
            },
            TestCase {
                num_rows: 5,
                answer: vec![
                    vec![1],
                    vec![1, 1],
                    vec![1, 2, 1],
                    vec![1, 3, 3, 1],
                    vec![1, 4, 6, 4, 1],
                ],
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::generate(c.num_rows);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
