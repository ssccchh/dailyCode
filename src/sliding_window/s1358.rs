use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut map = HashMap::<char, usize>::new();
        let (mut left, mut ans) = (0, 0);
        let s_chars: Vec<char> = s.chars().collect();

        for (_right, &char) in s_chars.iter().enumerate() {
            *map.entry(char).or_insert(0) += 1;
            while map.get(&'a').unwrap_or(&0) > &0
                && map.get(&'b').unwrap_or(&0) > &0
                && map.get(&'c').unwrap_or(&0) > &0
            {
                if let Some(count) = map.get_mut(&s_chars[left]) {
                    *count -= 1;
                }
                left += 1;
            }
            ans += left;
        }

        ans as i32
    }
}
