//! ## 对称二叉树
//!
//! 判断一个二叉树是否对称。
//!
//! **限制条件：**
//! - 树中节点数目在范围 `[1, 1000]` 内
//! - $-100 \le Node.val \le 100$

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    /// 使用层序遍历然后看每层的值的数组是否为对称的数组即可
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let mut layer_nodes = vec![Some(Rc::clone(&root.unwrap()))];
        while layer_nodes.iter().any(|e| e.is_some()) {
            let mut next_layer_nodes = vec![];
            let mut cur_layer_vals = vec![];
            for node in layer_nodes.into_iter() {
                if let Some(n) = node {
                    cur_layer_vals.push(Some(n.borrow().val));
                    next_layer_nodes.push(n.borrow().left.clone());
                    next_layer_nodes.push(n.borrow().right.clone());
                } else {
                    cur_layer_vals.push(None);
                    next_layer_nodes.push(None);
                    next_layer_nodes.push(None);
                }
            }

            for i in 0..(cur_layer_vals.len() / 2) {
                if cur_layer_vals[i] != cur_layer_vals[cur_layer_vals.len() - 1 - i] {
                    return false;
                }
            }

            layer_nodes = next_layer_nodes;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        answer: bool,
    }

    impl TestCase {
        pub fn from_vecs(nodes: Vec<Option<i32>>, answer: bool) -> Self {
            let root = TreeNode::try_from(nodes)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            Self { root, answer }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::from_vecs(
                vec![
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(4),
                    Some(3),
                ],
                true,
            ),
            TestCase::from_vecs(
                vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)],
                false,
            ),
            TestCase::from_vecs(vec![Some(1)], true),
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::is_symmetric(c.root);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
