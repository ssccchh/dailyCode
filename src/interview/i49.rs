use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();

        for str in strs {
            // 把单词按字母排序
            let mut sorted_str = str.clone().into_bytes();
            sorted_str.sort_unstable();
            map.entry(sorted_str).or_insert(vec![]).push(str);
        }

        map.into_values().collect()
    }
}
