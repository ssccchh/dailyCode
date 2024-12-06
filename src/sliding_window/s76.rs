use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map = HashMap::<u8, isize>::new();
        let n = s.len();
        let (s_byte, t_byte) = (s.as_bytes(), t.as_bytes());
        let (mut ans_left, mut ans_right) = (0, n + 1);
        let mut less = t.len();

        for i in 0..t_byte.len() {
            *map.entry(t_byte[i]).or_insert(0) += 1;
        }

        let mut left = 0;

        for (right, byte) in s_byte.iter().enumerate() {
            if let Some(count) = map.get_mut(byte) {
                *count -= 1;
                if *count == 0 {
                    less -= 1;
                }
            }

            while less == 0 {
                if right - left < ans_right - ans_left {
                    ans_right = right;
                    ans_left = left;
                }
                if let Some(count) = map.get_mut(&s_byte[left]) {
                    *count += 1;
                    if *count > 0 {
                        less += 1;
                    }
                }
                left += 1
            }
        }

        if ans_right < s.len() {
            s[ans_left..=ans_right].to_string()
        } else {
            String::from("")
        }
    }
}
