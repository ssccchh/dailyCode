use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut i, mut j) = (0, nums.len() as i32 - 1);

        while i <= j {
            let m = (j + i) / 2;
            if nums[m as usize] > target {
                j = m - 1;
            } else if nums[m as usize] < target {
                i = m + 1;
            } else {
                return m;
            }
        }
        i
    }
}
