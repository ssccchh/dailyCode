use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);
        let mut ans = 0;
        let s = s.as_bytes();

        for i in 1..s.len() {
            let x = map[&s[i - 1]];
            let y = map[&s[i]];

            if x >= y {
                ans += x;
            } else {
                ans -= x;
            }
        }

        ans
    }
}
