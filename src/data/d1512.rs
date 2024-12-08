use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, usize>::new();
        let mut ans = 0;
        for j in 0..nums.len() {
            if let Some(&c) = map.get(&nums[j]) {
                ans += c;
            }
            *map.entry(nums[j]).or_insert(0) += 1;
        }
        ans as i32
    }
}
