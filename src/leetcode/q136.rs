//! ## 只出现一次的数字
//!
//! 给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
//! 你必须设计并实现线性时间复杂度的算法来解决此问题，且该算法只使用常量额外空间。
//!
//! **限制条件：**
//! - $1 \le nums.length \le 3 * 10^4$
//! - $-3 * 10^4 \le nums[i] \le 3 * 10^4$

pub struct Solution;

impl Solution {
    /// 其余每个元素均出现两次，只有一个元素出现一次，使用异或
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, n| acc ^ n)
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
                nums: vec![2, 2, 1],
                answer: 1,
            },
            TestCase {
                nums: vec![4, 1, 2, 1, 2],
                answer: 4,
            },
            TestCase {
                nums: vec![1],
                answer: 1,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::single_number(c.nums);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
