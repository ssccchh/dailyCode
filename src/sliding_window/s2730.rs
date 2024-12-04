use crate::Solution;

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let (mut left, mut ans, mut dup_count) = (0, 0, 0);
        let s = s.as_bytes();

        for right in 0..s.len() {
            if right > 0 && s[right] == s[right - 1] {
                dup_count += 1;
            }
            while dup_count > 1 {
                if s[left] == s[left + 1] {
                    dup_count -= 1;
                }
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
