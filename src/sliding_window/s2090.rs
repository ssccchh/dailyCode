use crate::Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let mut ans: Vec<i64> = vec![-1; nums.len()];
        let mut sum = 0;
        let k = k as usize;

        for (i, &num) in nums.iter().enumerate() {
            sum += num as i64;

            if i >= k * 2 {
                let count = k * 2 + 1;
                ans[i - k] = ((sum as i64 / count as i64) as f64).floor() as i64;
                sum -= nums[i - k * 2] as i64
            }
        }
        ans
    }
}
