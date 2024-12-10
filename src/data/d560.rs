use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::<i32, usize>::new();
        let mut pre_sum = 0;
        let mut ans = 0;
        map.insert(0, 1);
        for num in nums {
            pre_sum += num;
            if let Some(x) = map.get(&(pre_sum - k)) {
                ans += x;
            }
            *map.entry(pre_sum).or_insert(0) += 1;
        }

        ans as i32
    }
}
