use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s_words: Vec<&str> = s.split(' ').collect();
        let mut pattern_map = HashMap::new();
        let mut s_map = HashMap::new();
        let pattern = pattern.as_bytes();

        if s_words.len() != pattern.len() {
            return false;
        }

        for i in 0..s_words.len() {
            // 正向检查
            if let Some(&word) = s_map.get(&pattern[i]) {
                if word != s_words[i] {
                    return false;
                }
            }

            // 反向检查
            if let Some(&s) = pattern_map.get(&s_words[i]) {
                if s != pattern[i] {
                    return false;
                }
            }

            // 检查通过，更新hash
            s_map.insert(pattern[i], s_words[i]);
            pattern_map.insert(s_words[i], pattern[i]);
        }

        true
    }
}
