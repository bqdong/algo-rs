//! ## 将有序数组转化为二叉搜索树
//!
//! 给定一个数组`nums`，已经是一个升序数组了，将其转换为平衡二叉搜索树。
//!
//! **平衡二叉树**是指该树所有节点的左右子树高度差不超过 1 。
//!
//! **限制条件：**
//! - $1 \le nums.length \le 10^4$
//! - $-10^4 \le nums[i] \le 10^4$
//! - `nums` 按严格递增顺序排列

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(_nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        nums: Vec<i32>,
        answer: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TestCase {
        pub fn from_vecs(nums: Vec<i32>, answer: Vec<Option<i32>>) -> Self {
            let an = TreeNode::try_from(answer)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            Self { nums, answer: an }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::from_vecs(
                vec![-10, -3, 0, 5, 9],
                vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)],
            ),
            TestCase::from_vecs(vec![1, 3], vec![Some(1), None, Some(3)]),
        ]
    }

    #[test]
    #[ignore = "It's hard"]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::sorted_array_to_bst(c.nums);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
