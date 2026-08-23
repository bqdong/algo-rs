//! ## 多数元素
//!
//! 给定一个数组，返回数组中出现次数大于 $\lfloor \frac{nums.length}{2} \rfloor$ 的那个元素。
//! $\lfloor n \rfloor$ 是指向下取整。
//!
//! **限制条件：**
//! - 数组中必定存在这样的数
//! - $1 \le nums.length \le 10^5$
//! - $-10^9 \le nums[i] \le 10^9$

pub struct Solution;

impl Solution {
    /// 不管复杂度的话，很容易做：
    /// ```
    /// use std::collections::HashMap;
    ///
    /// fn majority_element(nums: Vec<i32>) -> i32 {
    ///     let counter = nums.iter().fold(HashMap::new(), |mut acc, e| {
    ///         if acc.contains_key(e) {
    ///             let n = acc.get(e).unwrap();
    ///             acc.insert(e, n + 1);
    ///         } else {
    ///             acc.insert(e, 1);
    ///         }
    ///         return acc;
    ///     });
    ///     counter
    ///         .into_iter()
    ///         .max()
    ///         .expect("The number should exist")
    ///         .0
    ///         .to_owned()
    /// }
    /// ```
    ///
    /// 如果要换成时间复杂度为 $O(n)$ 和空间复杂度为 $O(1)$，可以这么做：
    /// > 要找的结果是多数元素，个数超过数组长度的一半，可以遍历数组一遍，如果前后
    /// > 两个元素不相同，则相互抵消，如果一样则保留，这样到最后肯定只剩多数元素了。
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((i32::MAX, 0), |acc, e| {
                if acc.1 == 0 {
                    return (e, 1);
                }
                if acc.0 == e {
                    (e, acc.1 + 1)
                } else {
                    (acc.0, acc.1 - 1)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        nums: Vec<i32>,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                nums: vec![3, 2, 3],
                answer: 3,
            },
            TestCase {
                nums: vec![2, 2, 1, 1, 1, 2, 2],
                answer: 2,
            },
            TestCase {
                nums: vec![3, 3, 4],
                answer: 3,
            },
            TestCase {
                nums: vec![3, 3, 4, 3],
                answer: 3,
            },
            TestCase {
                nums: vec![3],
                answer: 3,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |(idx, c): (usize, TestCase)| {
            let result = Solution::majority_element(c.nums);
            assert_eq!(c.answer, result, "The {} fails", idx);
        };
        cases.into_iter().enumerate().for_each(t);
    }
}
