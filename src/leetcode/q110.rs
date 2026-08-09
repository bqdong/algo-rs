//! ## 平衡二叉树
//!
//! 给定一个二叉树，判断它是否是平衡二叉树。
//!
//! **平衡二叉树**是指该树所有节点的左右子树高度差不超过 1 。
//!
//! **限制条件：**
//! - 树中的节点树在范围 `[0, 5000]` 内
//! - $-10^4 \le Node.val \le 10^4$

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_balanced(_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        answer: bool,
    }

    impl TestCase {
        pub fn from_vecs(root: Vec<Option<i32>>, answer: bool) -> Self {
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
                true,
            ),
            TestCase::from_vecs(
                vec![
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    None,
                    None,
                    Some(4),
                    Some(4),
                ],
                false,
            ),
            TestCase::default(),
        ]
    }

    #[test]
    #[ignore = "It's hard"]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::is_balanced(c.root);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
