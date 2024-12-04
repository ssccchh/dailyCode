use crate::Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let (mut left, mut ans, mut cost) = (0, 0, 0);
        let (s, t) = (s.as_bytes(), t.as_bytes());

        for right in 0..s.len() {
            cost += (t[right] as i32 - s[right] as i32).abs();
            while cost > max_cost {
                cost -= (t[left] as i32 - s[left] as i32).abs();
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}
