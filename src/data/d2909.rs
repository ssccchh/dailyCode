use crate::Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = i32::MAX;
        let mut suf = vec![0; n];

        // 先预计算出后缀的最小值
        suf[n - 1] = nums[n - 1];
        for i in (2..n - 1).rev() {
            suf[i] = suf[i + 1].min(nums[i])
        }

        // 再预计算出前缀的最小值
        let mut pre = nums[0];
        for j in 1..n - 1 {
            // 找到山子形最小的和
            if nums[j] > pre && nums[j] > suf[j + 1] {
                ans = ans.min(nums[j] + pre + suf[j + 1])
            }
            pre = pre.min(nums[j]);
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
