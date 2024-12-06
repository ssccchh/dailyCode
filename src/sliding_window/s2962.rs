use crate::Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let (mut ans, mut left, mut max, mut count) = (0, 0, 0, 0);
        if let Some(&x) = nums.iter().max() {
            max = x;
        }
        for right in 0..nums.len() {
            if nums[right] == max {
                count += 1;
            }
            while count >= k {
                if nums[left] == max {
                    count -= 1;
                }
                left += 1;
            }
            ans += left;
        }
        ans as i64
    }
}
