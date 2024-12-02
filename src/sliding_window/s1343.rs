use crate::Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        let k = k as usize;
        for (i, &num) in arr.iter().enumerate() {
            sum = sum + num;

            if i < k - 1 {
                continue;
            }

            let cur_threshold = sum / k as i32;
            if cur_threshold >= threshold {
                ans += 1;
            }

            sum -= arr[i + 1 - k]
        }

        ans
    }
}
