//! ## 路径总和
//!
//! 给定一个二叉树根节点和一个目标数，判断是否有一条从根节点到叶子节点
//! 的路径，使得路径上所有数字相加等于目标数。
//!
//! **限制条件：**
//! - 树中的节点树在范围 $[0, 5000]$ 内
//! - $-1000 \le Node.val \le 1000$
//! - $-1000 \le targetNum \le 1000$

use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::TreeNode;

pub struct Solution;

impl Solution {
    /// 递归解决：问题相当于左右子树中是否存在一条路径的数字只和等于 `target_num`
    /// 减去当前节点的数字
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        // 空的树总是不满足
        if root.is_none() {
            return false;
        }

        let root_unwrap = root.unwrap();
        let root_borrow = root_unwrap.borrow();
        let left_tree = root_borrow.left.clone();
        let right_tree = root_borrow.right.clone();
        if left_tree.is_none() && right_tree.is_none() {
            return root_borrow.val == target_sum;
        }

        let left_num = target_sum - root_borrow.val;
        Solution::has_path_sum(left_tree, left_num) || Solution::has_path_sum(right_tree, left_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        target_num: i32,
        answer: bool,
    }

    impl TestCase {
        pub fn from_vecs(root: Vec<Option<i32>>, target_num: i32, answer: bool) -> Self {
            let t = TreeNode::try_from(root)
                .ok()
                .map(|t| Rc::new(RefCell::new(t)));
            Self {
                root: t,
                target_num,
                answer,
            }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::from_vecs(
                vec![
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    None,
                    Some(1),
                ],
                22,
                true,
            ),
            TestCase::from_vecs(vec![Some(1), Some(2), Some(3)], 5, false),
            TestCase::default(), // 树是空的，所以不存在
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::has_path_sum(c.root, c.target_num);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
