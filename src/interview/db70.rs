use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 || n == 2 {
            return n;
        }

        let mut dp = vec![-1; n as usize + 1];
        dp[1] = 1;
        dp[2] = 2;

        for r in 3..dp.len() {
            dp[r] = dp[r - 1] + dp[r - 2]
        }

        dp[n as usize]
    }
}
