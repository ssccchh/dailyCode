use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut s_map = HashMap::new();

        if s.len() != t.len() {
            return false;
        }

        for i in 0..s.len() {
            *s_map.entry(s[i]).or_insert(0) += 1;
        }

        for i in 0..t.len() {
            if let Some(count) = s_map.get_mut(&t[i]) {
                *count -= 1;
                if *count < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
