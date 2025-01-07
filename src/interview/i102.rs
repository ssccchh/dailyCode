use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new(); // 队列
        let mut ans = Vec::new();

        if let Some(root) = root {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let n = queue.len();
            let mut vals = Vec::with_capacity(n);
            for _ in 0..n {
                if let Some(node) = queue.pop_front() {
                    let mut x = node.borrow_mut();
                    vals.push(x.val);

                    if let Some(left) = x.left.take() {
                        queue.push_back(left);
                    }

                    if let Some(right) = x.right.take() {
                        queue.push_back(right);
                    }
                }
            }

            ans.push(vals);
        }

        ans
    }
}
