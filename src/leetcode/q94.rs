//! ## 二叉树的中序遍历
//!
//! 给定一个二叉树的根节点，返回它的中序遍历。
//!
//! **限制条件：**
//! - 树中节点数目在范围 `[0, 100]` 内
//! - $-100 \le Node.val \le 100$

use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl TryFrom<Vec<Option<i32>>> for TreeNode {
    type Error = &'static str;

    /// 从 `Vec` 构建树
    /// 若 `Vec` 中有一个元素为 `None`
    fn try_from(value: Vec<Option<i32>>) -> Result<Self, Self::Error> {
        if value.is_empty() || value[0].is_none() {
            return Err("Empty tree");
        }

        let root = Rc::new(RefCell::new(TreeNode::new(value[0].unwrap())));

        let mut idx = 1;
        let mut cur_layer_nodes = vec![Rc::downgrade(&root)];
        while !cur_layer_nodes.is_empty() {
            let mut next_layer_nodes = vec![];
            for node in cur_layer_nodes.iter_mut() {
                let n = node.upgrade().unwrap();

                if idx >= value.len() {
                    break;
                }
                n.borrow_mut().left = if value[idx].is_none() {
                    None
                } else {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(value[idx].unwrap())));
                    next_layer_nodes.push(Rc::downgrade(&new_node));
                    Some(new_node)
                };
                idx += 1;

                if idx >= value.len() {
                    break;
                }
                n.borrow_mut().right = if value[idx].is_none() {
                    None
                } else {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(value[idx].unwrap())));
                    next_layer_nodes.push(Rc::downgrade(&new_node));
                    Some(new_node)
                };
                idx += 1;
            }
            cur_layer_nodes = next_layer_nodes;
        }

        Ok(Rc::try_unwrap(root).ok().map(RefCell::into_inner).unwrap())
    }
}

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
