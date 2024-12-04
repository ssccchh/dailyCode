use std::i32::MAX;

use crate::Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut ans, mut sum) = (0, MAX, 0);
        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                ans = ans.min((right - left + 1) as i32);
                sum -= nums[left];
                left += 1;
            }
        }
        if ans == MAX {
            0
        } else {
            ans
        }
    }
}
