use crate::Solution;

impl Solution {
    pub fn count_subarrays2(nums: Vec<i32>, k: i64) -> i64 {
        let (mut ans, mut left, mut sum) = (0, 0, 0);

        for right in 0..nums.len() {
            sum += nums[right] as i64;

            while sum * (right - left + 1) as i64 >= k {
                sum -= nums[left] as i64;
                left += 1;
            }

            ans += (right - left + 1) as i64;
        }

        ans
    }
}
