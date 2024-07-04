use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len() - 1;

        while i <= j {
            let m = i + (j - i) / 2;
            if nums[m] > target {
                j = m - 1;
            } else if nums[m] < target {
                i = m + 1;
            } else {
                return m as i32;
            }
        }

        return -1;
    }
}
