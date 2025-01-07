use std::rc::Rc;
use std::{cell::RefCell, collections::VecDeque};

use crate::{Solution, TreeNode};
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // 中序遍历，排序
        let mut stack = VecDeque::new();
        let mut current_node = root;
        let mut result = Vec::new();
        while current_node.is_some() || !stack.is_empty() {
            while let Some(node) = current_node {
                stack.push_back(node.clone()); // 节点入栈
                current_node = node.borrow().left.clone(); // 一直向下查左子树
            }

            // 弹出栈顶，并且访问节点的值
            if let Some(node) = stack.pop_back() {
                let node = node.borrow();
                result.push(node.val);
                current_node = node.right.clone(); // 处理完左子树，转向右子树
            }
        }
        result[k as usize - 1]
    }
}
