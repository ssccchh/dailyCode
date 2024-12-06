use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();

        for (j, &num) in nums.iter().enumerate() {
            if let Some(&i) = map.get(&(target - num)) {
                return vec![i as i32, j as i32];
            }

            map.insert(num, j);
        }

        unreachable!()
    }
}
