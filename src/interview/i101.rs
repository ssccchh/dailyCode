use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_same(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    let mut pb = p.borrow_mut();
                    let mut qb = q.borrow_mut();

                    pb.val == qb.val
                        && is_same(pb.left.take(), qb.right.take())
                        && is_same(pb.right.take(), qb.left.take())
                }
                (None, None) => true,
                _ => false,
            }
        }

        if let Some(node) = root {
            is_same(node.borrow().left.clone(), node.borrow().right.clone())
        } else {
            false
        }
    }
}
