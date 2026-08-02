//! ## 合并两个有序数组
//!
//! 两个非递减数组 `nums1` 和 `nums2` 合并后仍需要按照非递减顺序，
//! 结果存储在 `nums1 数组中。`m` 和 `n` 分别为两个数组的元素个数。
//!
//! **限制条件：**
//! - $nums1.length = m + n$
//! - $nums2.lenght = n$
//! - $0 <= m, n <= 200$
//! - $1 <= m + n <= 200$
//! - $ -10^9 <= nums1[i], nums2[j] <= 10^9 $

pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in m..(m + n) {
            nums1[i as usize] = nums2[(i - m) as usize];
        }

        nums1.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        nums1: Vec<i32>,
        m: i32,
        nums2: Vec<i32>,
        n: i32,
        answer: Vec<i32>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                nums1: vec![1, 2, 3, 0, 0, 0],
                m: 3,
                nums2: vec![2, 5, 6],
                n: 3,
                answer: vec![1, 2, 2, 3, 5, 6],
            },
            TestCase {
                nums1: vec![1],
                m: 1,
                nums2: vec![],
                n: 0,
                answer: vec![1],
            },
            TestCase {
                nums1: vec![0],
                m: 0,
                nums2: vec![1],
                n: 1,
                answer: vec![1],
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |mut c: TestCase| {
            Solution::merge(&mut c.nums1, c.m, &mut c.nums2, c.n);
            assert_eq!(c.answer, c.nums1);
        };
        cases.into_iter().for_each(t);
    }
}
