use crate::Solution;

impl Solution {
    pub fn _remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow_index = 0;
        for fast_index in 1..nums.len() {
            if (nums[fast_index] == nums[slow_index]) {
                continue;
            } else {
                slow_index += 1;
                nums[slow_index] = nums[fast_index];
            }
        }
        slow_index as i32 + 1
    }
}
