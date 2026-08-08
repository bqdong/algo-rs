//! ## 二叉树的最大深度
//!
//! 给定一个二叉树，返回其最大深度。
//!
//! **限制条件：**
//! - 树中节点数目在范围 `[0, 10000]` 内
//! - $-100 \le Node.val \le 100$

use std::{cell::RefCell, cmp::max, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = root {
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                return 1;
            }

            let left_depth = Solution::max_depth(n.borrow().left.clone());
            let right_depth = Solution::max_depth(n.borrow().right.clone());
            return max(left_depth, right_depth) + 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        answer: i32,
    }

    impl TestCase {
        pub fn from_vecs(nodes: Vec<Option<i32>>, answer: i32) -> Self {
            let root = TreeNode::try_from(nodes)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            Self { root, answer }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::from_vecs(
                vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
                3,
            ),
            TestCase::from_vecs(vec![Some(1), None, Some(2)], 2),
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::max_depth(c.root);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
