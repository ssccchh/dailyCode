use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let l_depth = Self::max_depth(node.left.take());
            let r_depth = Self::max_depth(node.right.take());

            return l_depth.max(r_depth) + 1;
        }
        0
    }
}
