use crate::Solution;

impl Solution {
    pub fn lower_bound(nums: Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    pub fn search_insert1(nums: Vec<i32>, target: i32) -> i32 {
        Self::lower_bound(nums, target) as _
    }
}
