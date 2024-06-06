use crate::Solution;

impl Solution {
    pub fn _remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow_index = 0;
        for fast_index in 0..nums.len() {
            if nums[fast_index] != val {
                nums[slow_index] = nums[fast_index];
                slow_index += 1;
            }
        }

        return slow_index as i32;
    }
}
