//! ## 搜索插入位置
//!
//! 给一个升序数组和目标值，找到目标值对应的索引，
//! 如果找不到，找到其插入的索引。
//!
//! 必须使用时间复杂度为 O(log n) 的算法。
//!
//! 限制条件：
//! - 1 <= nums.length <= 10^4
//! - -10^4 <= nums[i], target <= 10^4
//! - nums 中的元素没有重复且升序排列

pub struct Solution;

impl Solution {
    /// 二分查找
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        assert!(nums.is_sorted(), "Illegal params");

        if nums.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let middle = left + (right - left) / 2;
            if nums[middle] > target {
                right = middle - 1;
            } else if nums[middle] < target {
                left = middle + 1;
            } else {
                return middle as i32;
            }
        }

        // After while, `left >= right`
        if left >= nums.len() {
            return left as i32;
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 5,
                answer: 2,
            },
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 2,
                answer: 1,
            },
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 7,
                answer: 4,
            },
            TestCase {
                nums: vec![1, 3],
                target: 0,
                answer: 0,
            },
            TestCase {
                nums: vec![1, 3],
                target: 1,
                answer: 0,
            },
            TestCase {
                nums: vec![1, 3],
                target: 2,
                answer: 1,
            },
            TestCase {
                nums: vec![1, 3],
                target: 3,
                answer: 2,
            },
            TestCase {
                nums: vec![1],
                target: 1,
                answer: 0,
            },
            TestCase {
                nums: vec![1],
                target: 2,
                answer: 1,
            },
            TestCase {
                nums: vec![1],
                target: 0,
                answer: 0,
            },
            TestCase {
                nums: vec![],
                target: 1,
                answer: 0,
            },
        ]
    }

    #[test]
    #[ignore = "It's hard"]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::search_insert(c.nums, c.target);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
