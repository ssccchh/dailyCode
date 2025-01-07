use std::rc::Rc;
use std::{cell::RefCell, collections::VecDeque};

use crate::{Solution, TreeNode};
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        // 深度优先遍历
        // let mut data: Vec<(i64, usize)> = vec![];

        // fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, data: &mut Vec<(i64, usize)>) {
        //     if let Some(node) = node {
        //         // 深度和层数相等，开辟下一层
        //         if depth == data.len() {
        //             data.push((node.borrow().val as i64, 1));
        //         } else {
        //             data[depth].0 += node.borrow().val as i64;
        //             data[depth].1 += 1;
        //         }

        //         // 深度优先
        //         dfs(node.borrow_mut().left.clone(), depth + 1, data);
        //         dfs(node.borrow_mut().right.clone(), depth + 1, data);
        //     }
        // }

        // dfs(root, 0, &mut data);

        // data.iter()
        //     .map(|(sum, count)| *sum as f64 / *count as f64)
        //     .collect()

        // 广度优先遍历

        // 根节点入队列
        if root.is_none() {
            return vec![];
        }

        let mut data: Vec<(i64, usize)> = vec![];
        let mut queue = VecDeque::new();

        // 将根节点加入队列
        queue.push_back(root);

        // 广度优先遍历
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level_sum: i64 = 0;
            let mut level_count = 0;

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    if let Some(node) = node {
                        let node = node.borrow();
                        level_sum += node.val as i64;
                        // 将左右子节点加入队列
                        if let Some(left) = node.left.clone() {
                            queue.push_back(Some(left));
                        }
                        if let Some(right) = node.right.clone() {
                            queue.push_back(Some(right));
                        }
                    }
                    level_count += 1;
                }
            }

            // 记录当前层的和与节点数量
            data.push((level_sum as i64, level_count));
        }

        // 计算每一层的平均值
        data.iter()
            .map(|(sum, count)| (*sum as f64 / *count as f64))
            .collect()
    }
}
