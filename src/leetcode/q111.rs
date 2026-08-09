//! ## 二叉树的最小深度
//!
//! 最小深度是指从根节点到叶子节点的最短路径上的节点数量。
//!
//! **限制条件：**
//! - 树中的节点树在范围 $[0, 10^5]$ 内
//! - $-1000 \le Node.val \le 1000$

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    /// 层序遍历解决：从第一层开始，遍历过程中如果遇到一个节点没有左右子节点，
    /// 即其为叶子节点，此时层数就为最小深度
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut cur_layer = 1;
        let mut cur_nodes = vec![Some(root.unwrap().clone())];
        'outer: loop {
            let mut next_layer_nodes = vec![];
            for node in cur_nodes {
                let binding = node.unwrap();
                let node_borrowed = binding.borrow();
                if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                    break 'outer;
                }

                if node_borrowed.left.is_some() {
                    next_layer_nodes.push(node_borrowed.left.clone());
                }
                if node_borrowed.right.is_some() {
                    next_layer_nodes.push(node_borrowed.right.clone());
                }
            }
            cur_nodes = next_layer_nodes;
            cur_layer += 1;
        }

        cur_layer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        answer: i32,
    }

    impl TestCase {
        pub fn from_vecs(root: Vec<Option<i32>>, answer: i32) -> Self {
            let t = TreeNode::try_from(root)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            Self { root: t, answer }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::from_vecs(
                vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
                2,
            ),
            TestCase::from_vecs(
                vec![
                    Some(2),
                    None,
                    Some(3),
                    None,
                    Some(4),
                    None,
                    Some(5),
                    None,
                    Some(6),
                ],
                5,
            ),
            TestCase::default(),
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::min_depth(c.root);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
