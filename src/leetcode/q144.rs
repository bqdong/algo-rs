//! ## 二叉树的前序遍历
//!
//! 给定二叉树的根节点 root，返回它节点值的前序遍历。
//!
//! **限制条件：**
//! - 树中节点数目在范围`[0, 100]`内
//! - $-100 \le Node.val \le 100$

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let tr = root.unwrap();
        let mut result = vec![tr.borrow().val];
        let left_root = tr.borrow().left.clone();
        result.append(&mut Solution::preorder_traversal(left_root));
        let right_root = tr.borrow().right.clone();
        result.append(&mut Solution::preorder_traversal(right_root));

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::common::to_tree;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        answer: Vec<i32>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                root: to_tree(vec![Some(1), None, Some(2), Some(3)]),
                answer: vec![1, 2, 3],
            },
            TestCase {
                root: to_tree(vec![
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    None,
                    Some(8),
                    None,
                    None,
                    Some(6),
                    Some(7),
                    Some(9),
                ]),
                answer: vec![1, 2, 4, 5, 6, 7, 3, 8, 9],
            },
            TestCase {
                root: to_tree(vec![]),
                answer: vec![],
            },
            TestCase {
                root: to_tree(vec![Some(1)]),
                answer: vec![1],
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::preorder_traversal(c.root);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
