use crate::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut zero_count = 0;

        for (i, &num) in nums.iter().enumerate() {
            if num == 0 {
                zero_count += 1;
            }
            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            ans = ans.max(i - left);
        }

        ans as i32
    }
}
