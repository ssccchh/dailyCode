use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut suf = vec![0; n];

        suf[n - 1] = 1; // 初始值
        for i in (0..n - 1).rev() {
            suf[i] = suf[i + 1] * nums[i + 1];
        }

        let mut pre = 1;
        for (i, x) in nums.iter().enumerate() {
            suf[i] *= pre;
            pre *= x;
        }

        suf
    }
}
