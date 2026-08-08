//! ## 二叉树的中序遍历
//!
//! 给定一个二叉树的根节点，返回它的中序遍历。
//!
//! **限制条件：**
//! - 树中节点数目在范围 `[0, 100]` 内
//! - $-100 \le Node.val \le 100$

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    /// 中序遍历二叉树
    /// 使用递归的方法：
    /// - 遍历左子树
    /// - 当前节点
    /// - 遍历右子树
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut result = vec![];

        let tree_node = root.unwrap();

        // traverse left sub-tree
        let mut left = Solution::inorder_traversal(tree_node.borrow().left.clone());
        result.append(&mut left);

        result.push(tree_node.borrow().val);

        // traverse right sub-tree
        let mut right = Solution::inorder_traversal(tree_node.borrow().right.clone());
        result.append(&mut right);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        answer: Vec<i32>,
    }

    impl TestCase {
        pub fn from_vecs(root: Vec<Option<i32>>, answer: Vec<i32>) -> Self {
            let tree = TreeNode::try_from(root)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            Self { root: tree, answer }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::from_vecs(vec![Some(1), None, Some(2), Some(3)], vec![1, 3, 2]),
            TestCase::from_vecs(vec![Some(1)], vec![1]),
            TestCase::from_vecs(vec![], vec![]),
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::inorder_traversal(c.root);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
