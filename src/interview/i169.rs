use crate::Solution;

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        let mut slow_index = 0;
        let mut result = 0;
        nums.sort();
        for fast_index in 0..nums.len() {
            if nums[slow_index] != nums[fast_index] {
                slow_index = fast_index
            } else if fast_index - slow_index + 1 > nums.len() / 2 {
                result = nums[slow_index]
            }
        }
        result
    }
}
