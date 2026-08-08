//! ## 相同的树
//!
//! 判断两棵树是否相同（二叉树）
//!
//! **限制条件：**
//! - 树中节点数目在范围 `[0, 100]` 内
//! - $-100 \le Node.val \le 100$

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.as_ref().xor(q.as_ref()).is_some() {
            return false;
        }

        let binding1 = p.unwrap();
        let binding2 = q.unwrap();
        let p_borrow = binding1.borrow();
        let q_borrow = binding2.borrow();

        if p_borrow.val != q_borrow.val {
            return false;
        }

        let left_same = Solution::is_same_tree(p_borrow.left.clone(), q_borrow.left.clone());
        let right_same = Solution::is_same_tree(p_borrow.right.clone(), q_borrow.right.clone());

        left_same && right_same
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
        answer: bool,
    }

    impl TestCase {
        pub fn from_vecs(t1: Vec<Option<i32>>, t2: Vec<Option<i32>>, answer: bool) -> Self {
            let tree1 = TreeNode::try_from(t1)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            let tree2 = TreeNode::try_from(t2)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            Self {
                t1: tree1,
                t2: tree2,
                answer,
            }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::from_vecs(
                vec![Some(1), None, Some(2), Some(3)],
                vec![Some(1), None, Some(2), Some(3)],
                true,
            ),
            TestCase::from_vecs(
                vec![Some(1), Some(2), Some(3)],
                vec![Some(1), Some(2), Some(3)],
                true,
            ),
            TestCase::from_vecs(vec![Some(1), Some(2)], vec![Some(1), None, Some(2)], false),
            TestCase::from_vecs(
                vec![Some(1), Some(2), Some(1)],
                vec![Some(1), Some(1), Some(2)],
                false,
            ),
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::is_same_tree(c.t1, c.t2);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
