//! ## [Excel 表序列号](https://leetcode.cn/problems/excel-sheet-column-number/description/)
//!
//! 给一个字符串，表示 Excel 表格中的列名称，返回该列名称对应的序列号。
//!
//! ```text
//! A -> 1
//! B -> 2
//! C -> 3
//! ...
//! Z -> 26
//! AA -> 27
//! AB -> 28
//! ...
//! ```
//!
//! **限制条件：**
//! - $1 \l3 columnTitle.length \le 7$
//! - `columnTitle` 仅由大写英文组成
//! - `columnTitle` 在范围 `["A", "FXSHRXW"]` 内

pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let c2n = |c| c as u32 - 'A' as u32 + 1;
        column_title
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, c| acc + c2n(c.1) * 26_u32.pow((c.0) as u32)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        column_title: String,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                column_title: "A".to_string(),
                answer: 1,
            },
            TestCase {
                column_title: "Z".to_string(),
                answer: 26,
            },
            TestCase {
                column_title: "AZ".to_string(),
                answer: 52,
            },
            TestCase {
                answer: 28,
                column_title: "AB".to_string(),
            },
            TestCase {
                answer: 701,
                column_title: "ZY".to_string(),
            },
            TestCase {
                answer: 2147483647,
                column_title: "FXSHRXW".to_string(),
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::title_to_number(c.column_title);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
