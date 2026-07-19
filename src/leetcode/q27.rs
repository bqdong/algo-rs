//! ## 移除元素
//!
//! 给一个数组 `nums` 和一个数 `val`，原地 `nums` 中的 `val`，并返回删除后数组中元素的数量。
//! 原地删除的数组中元素的顺序不限。
//!
//! 取值范围：
//! - 0 <= nums.length <= 100
//! - 0 <= nums[i] <= 50
//! - 0 <= val <= 100

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut head = 0;
        let mut tail = nums.len() - 1;
        while head <= tail {
            // 指向第一个尾部的非 val 的值，准备替换
            while nums[tail] == val {
                if tail == 0 {
                    break;
                }
                tail -= 1;
            }

            // 指向头部第一个 val，准备替换
            while nums[head] != val {
                if head == nums.len() - 1 {
                    break;
                }
                head += 1;
            }

            if head >= tail {
                break;
            }

            // 替换
            nums.swap(head, tail);

            if head < nums.len() - 1 {
                head += 1;
            }
            tail = tail.saturating_sub(1);
        }

        let bound = if nums[tail] == val {
            while tail > 0 && nums[tail] == val {
                tail -= 1;
            }
            if nums[tail] == val { 0 } else { tail + 1 }
        } else {
            tail + 1
        };
        nums.drain(bound..);
        nums.len().try_into().expect("Invalid argument")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        nums: Vec<i32>,
        val: i32,
        answer: (Vec<i32>, i32),
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                nums: vec![],
                val: 3,
                answer: (vec![], 0),
            },
            TestCase {
                nums: vec![3],
                val: 3,
                answer: (vec![], 0),
            },
            TestCase {
                nums: vec![3, 3],
                val: 5,
                answer: (vec![3, 3], 2),
            },
            TestCase {
                nums: vec![3, 2, 2, 3],
                val: 3,
                answer: (vec![2, 2], 2),
            },
            TestCase {
                nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
                val: 2,
                answer: (vec![0, 1, 4, 0, 3], 5),
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |mut c: TestCase| {
            let result = Solution::remove_element(&mut c.nums, c.val);
            assert_eq!(c.answer.1, result);

            c.nums.sort();
            c.answer.0.sort();
            assert_eq!(c.answer.0, c.nums);
        };
        cases.into_iter().for_each(t);
    }
}
