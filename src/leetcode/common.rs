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

pub fn to_tree(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    Some(TryInto::<TreeNode>::try_into(nums).unwrap()).map(|t| Rc::new(RefCell::new(t)))
}
