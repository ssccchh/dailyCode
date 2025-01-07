use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 前序遍历，中左右
        // 中序遍历，左中右
        // 思路：前序遍历找到根节点，再根据中序遍历找到左右子树，递归得到最终二叉树
        if preorder.is_empty() {
            return None;
        }
        let root = preorder[0];
        let position = inorder.iter().position(|val| *val == root);
        if let Some(p) = position {
            let in_left = inorder[..p].to_vec();
            let in_right = inorder[p + 1..inorder.len()].to_vec();
            let pre_left = preorder[1..1 + p].to_vec();
            let pre_right = preorder[1 + p..].to_vec();

            let left = Self::build_tree(pre_left, in_left);
            let right = Self::build_tree(pre_right, in_right);

            Some(Rc::new(RefCell::new(TreeNode {
                val: root,
                left,
                right,
            })))
        } else {
            None
        }
    }
}
