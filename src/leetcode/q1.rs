//! ## 两数之和
//!
//! 给定一个数组和一个数字，从数组中找出两个数，两数之和需等于
//! 给定的数字，返回这两个数的下标。
//!
//! 注意事项：
//! - 必定存在这样的两个数
//! - 存在多对，返回任意一对的下标即可（只存在一个答案）
//! - 返回的两个下标不能相同
//! - 返回的下标的顺序任意
//!
//! 取值范围：
//! - 2 <= nums.length <= 10^4
//! - -10^9 <= nums[i] <= 10^9
//! - -10^9 <= target <= 10^9

pub struct Solution;

impl Solution {
    /// 暴力解法：二层遍历找出正确答案，时间复杂度 O(n^2)
    /// 下标索引在 i32 范围内，i32 在正负 21 亿之间
    pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                // 加法不会溢出 i32 的表示范围
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!("Illegal state, there must be an answer but actually not!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        answer: Vec<i32>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                nums: vec![2, 7, 11, 15],
                target: 9,
                answer: vec![0, 1],
            },
            TestCase {
                nums: vec![3, 2, 4],
                target: 6,
                answer: vec![1, 2],
            },
            TestCase {
                nums: vec![3, 3],
                target: 6,
                answer: vec![0, 1],
            },
        ]
    }

    #[test]
    fn test_two_sums() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let mut result = Solution::two_sums(c.nums, c.target);
            assert!(result.len() == 2);

            result.sort();
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
