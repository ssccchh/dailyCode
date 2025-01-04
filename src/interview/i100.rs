use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let mut pb = p.borrow_mut();
                let mut qb = q.borrow_mut();
                pb.val == qb.val
                    && Self::is_same_tree(pb.left.take(), qb.left.take())
                    && Self::is_same_tree(pb.right.take(), qb.right.take())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
