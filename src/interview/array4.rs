use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow_index = 0;
        let mut fast_index = 1;
        let mut nums_len = nums.len();

        while nums_len > fast_index {
            if nums[fast_index] == nums[slow_index] && fast_index - slow_index > 1 {
                nums.remove(fast_index);
                nums_len -= 1;
            } else if nums[fast_index] != nums[slow_index] {
                slow_index = fast_index;
                fast_index += 1;
            } else {
                fast_index += 1;
            }
        }
        nums_len as i32
    }
}
