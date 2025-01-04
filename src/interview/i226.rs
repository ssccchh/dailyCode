use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node_clone = Rc::clone(&node);
            let mut node_borrow = node.borrow_mut();
            let left = Self::invert_tree(node_borrow.left.take());
            let right = Self::invert_tree(node_borrow.right.take());
            node_borrow.left = right;
            node_borrow.right = left;
            Some(node_clone)
        } else {
            None
        }
    }
}
