use std::i32::MIN;

use crate::Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut ans = MIN;
        let mut sum: i32 = 0;
        let k = k as usize;

        for (i, num) in nums.iter().enumerate() {
            sum = sum + num;
            if i < k - 1 {
                continue;
            }

            ans = ans.max(sum);

            sum = sum - nums[i + 1 - k];
        }

        let average: f64 = ans as f64 / k as f64;

        average
    }
}
