//! ## 删除有序数组中的重复项
//!
//! 给一个非严格递增的数组，原地删除重复的元素，使每个元素只出现一次。
//! 返回删除重复元素后的数组长度。元素的相对顺序应该保持一致。
//!
//! 取值范围：
//! - 1 <= nums.length <= 3 * 10^4
//! - -100 <= nums[i] <= 100

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;
        while idx < nums.len() {
            let mut end = idx + 1;
            while end < nums.len() && nums[idx] == nums[end] {
                end += 1;
            }
            if idx + 1 < end {
                nums.drain(idx + 1..end);
            } else {
                idx += 1;
            }
        }
        nums.len().try_into().expect("Illegal parameters!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        nums: Vec<i32>,
        answer: (Vec<i32>, i32),
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                nums: vec![1, 1, 2],
                answer: (vec![1, 2], 2),
            },
            TestCase {
                nums: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
                answer: (vec![0, 1, 2, 3, 4], 5),
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |mut c: TestCase| {
            let result = Solution::remove_duplicates(&mut c.nums);
            assert_eq!(c.answer.0, c.nums);
            assert_eq!(c.answer.1, result);
        };
        cases.into_iter().for_each(t);
    }
}
