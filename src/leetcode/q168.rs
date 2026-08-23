//! ## [Excel 表列名称](https://leetcode.cn/problems/excel-sheet-column-title/description/)
//!
//! 给你一个整数 `columnNumber`，返回它在 Excel 表中相对应的列名称。
//!
//! **限制条件：**
//! - $1 \le columnNumber \le 2^31-1$

pub struct Solution;

impl Solution {
    /// 把一个字符串转换 column_number 的过程：
    /// 设字符串长度为 n ($n \ge 1$)，则
    /// ```
    /// let s = "ABCEDE".to_string();
    /// let c2n = |c| c as u32 - 'A' as u32 + 1;
    /// let result: u32 = s
    ///     .chars()
    ///     .rev()
    ///     .enumerate()
    ///     .fold(0, |acc, c| acc + c2n(c.1).pow((s.len() - c.0) as u32));
    /// ```
    ///
    /// 十进制转换成十六进制：
    /// ```
    /// let n2c = |n| {
    ///     if n <= 9 {
    ///         char::from_u32(n)
    ///     } else {
    ///         char::from_u32('A' as u32 + n - 10)
    ///     }
    /// };
    ///
    /// let mut digit = 143408u32;
    /// let mut chars_rev = vec![];
    /// loop {
    ///     if digit < 16 {
    ///         chars_rev.push(n2c(digit));
    ///         break;
    ///     }
    ///     let r = digit % 16;
    ///     chars_rev.push(n2c(digit));
    ///
    ///     digit /= 16;
    /// }
    ///
    /// let result: String = chars_rev
    ///     .iter()
    ///     .rev()
    ///     .map(|e| e.as_ref().unwrap())
    ///     .collect();
    /// ```
    pub fn convert_to_title(column_number: i32) -> String {
        // convert digit to char
        // 1 -> A
        // 2 -> B
        // ...
        // 26 -> Z
        let n2c = |n: i32| char::from_u32('A' as u32 + n as u32 - 1).unwrap();

        let mut result = vec![];
        let rem = column_number % 26;
        if rem > 0 {
            result.push(n2c(rem));
        }
        let mut left = (column_number - rem) / 26; // 还剩下多少个 26
        loop {
            let rem = left % 26;
            if rem > 0 {
                result.push(n2c(rem));
            }
            left /= 26;
            if left == 0 {
                break;
            }
        }

        result.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        column_number: i32,
        answer: String,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                column_number: 1,
                answer: "A".to_string(),
            },
            TestCase {
                column_number: 26,
                answer: "Z".to_string(),
            },
            TestCase {
                column_number: 52,
                answer: "AZ".to_string(),
            },
            TestCase {
                column_number: 28,
                answer: "AB".to_string(),
            },
            TestCase {
                column_number: 701,
                answer: "ZY".to_string(),
            },
            TestCase {
                column_number: 2147483647,
                answer: "FXSHRXW".to_string(),
            },
        ]
    }

    #[test]
    #[ignore = "It's hard"]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::convert_to_title(c.column_number);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
