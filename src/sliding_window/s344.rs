use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut right = s.len() - 1;
        let mut cache_char: char;

        for left in 0..s.len() {
            if right >= left {
                cache_char = s[left];
                s[left] = s[right];
                s[right] = cache_char;
                right -= 1;
            }
        }
    }
}
