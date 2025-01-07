use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn build_tree1(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }

        let root = postorder.last().unwrap(); // 最后一位是根节点
        let left_size = inorder.iter().position(|val| *val == *root).unwrap();

        let in_left = inorder[..left_size].to_vec();
        let in_right = inorder[left_size + 1..inorder.len()].to_vec();
        let post_left = postorder[0..left_size].to_vec();
        let post_right = postorder[left_size + 1..postorder.len() - 1].to_vec();

        let left_tree = Self::build_tree1(in_left, post_left);
        let right_tree = Self::build_tree1(in_right, post_right);

        Some(Rc::new(RefCell::new(TreeNode {
            val: *root,
            left: left_tree,
            right: right_tree,
        })))
    }
}
