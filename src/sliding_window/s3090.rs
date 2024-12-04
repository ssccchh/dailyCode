use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut left = 0;
        let mut ans = 0;
        let mut map: HashMap<u8, i32> = HashMap::new();
        let s = s.as_bytes();

        for (i, &byte) in s.iter().enumerate() {
            *map.entry(byte).or_insert(0) += 1;

            while map[&byte] > 2 {
                let left_byte = s[left];
                *map.get_mut(&left_byte).unwrap() -= 1;
                left += 1;
            }

            ans = ans.max(i - left + 1);
        }
        ans as i32
    }
}
