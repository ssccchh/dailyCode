use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 二叉树中序遍历，递归
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, pre: &mut i32) {
            if let Some(node) = node {
                let node = node.borrow();
                dfs(&node.left, ans, pre);
                *ans = (*ans).min(node.val - *pre);
                *pre = node.val;
                dfs(&node.right, ans, pre);
            }
        }

        let mut ans = i32::MAX;
        let mut pre = i32::MIN / 2;

        dfs(&root, &mut ans, &mut pre);
        ans
    }
}
