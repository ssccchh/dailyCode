use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        for (j, &num) in nums.iter().enumerate() {
            if let Some(&i) = map.get(&num) {
                if j - i <= k as usize {
                    return true;
                }
            }
            map.insert(num, j);
        }
        false
    }
}
