use crate::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // 双指针 + 前后缀分解
        // let n = height.len();
        // let (mut left, mut right, mut ans) = (0, n - 1, 0);
        // let (mut pre_max, mut suf_max) = (height[0], height[n - 1]);

        // while left < right {
        //     if pre_max < suf_max {
        //         ans += pre_max - height[left];
        //         left += 1;
        //         pre_max = pre_max.max(height[left]);
        //     } else {
        //         ans += suf_max - height[right];
        //         right -= 1;
        //         suf_max = suf_max.max(height[right]);
        //     }
        // }
        // ans

        // 前后缀分解
        let n = height.len();
        if n == 0 {
            return 0;
        }

        let mut pre_max = vec![0; n];
        let mut suf_max = vec![0; n];
        let mut ans = 0;

        pre_max[0] = height[0];
        for i in 1..n {
            pre_max[i] = height[i].max(pre_max[i - 1]);
        }

        suf_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            suf_max[i] = height[i].max(suf_max[i + 1]);
        }

        for i in 0..n {
            ans += pre_max[i].min(suf_max[i]) - height[i];
        }

        ans
    }
}
