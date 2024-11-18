use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut index = 0;
        let s = s.as_bytes();
        for j in t.as_bytes() {
            if s[index] == *j {
                index += 1;

                if index == s.len() {
                    return true;
                }
            }
        }

        false
    }
}
