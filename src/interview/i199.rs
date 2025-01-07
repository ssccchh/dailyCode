use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, ans: &mut Vec<i32>) {
            if let Some(node) = node {
                if ans.len() == depth {
                    ans.push(node.borrow().val);
                }
                dfs(node.borrow().right.clone(), depth + 1, ans);
                dfs(node.borrow().left.clone(), depth + 1, ans);
            }
        }

        dfs(root, 0, &mut ans);
        ans
    }
}
