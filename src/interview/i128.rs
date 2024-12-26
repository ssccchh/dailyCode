use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hash = nums.into_iter().collect::<HashSet<_>>();

        let mut result = 0;
        for &start in &hash {
            // 如果有比num小1的数，那么以num - 1为起点进行查询
            if hash.contains(&(start - 1)) {
                continue;
            }

            // 设置终点为num + 1，不断往后查终点
            let mut end = start + 1;
            while hash.contains(&end) {
                end += 1;
            }
            result = result.max(end - start)
        }
        result
    }
}
