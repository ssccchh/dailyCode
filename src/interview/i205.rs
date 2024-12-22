use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();
        for i in 0..s.len() {
            if let Some(t_byte) = s_map.get(&s[i]) {
                if *t_byte != t[i] {
                    return false;
                }
            } else {
                s_map.insert(s[i], t[i]); // 正向
            }

            if let Some(s_byte) = t_map.get(&t[i]) {
                if *s_byte != s[i] {
                    return false;
                }
            } else {
                t_map.insert(t[i], s[i]);
            }
        }

        true
    }
}
