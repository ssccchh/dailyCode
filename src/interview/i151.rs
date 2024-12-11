use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s = s.trim().as_bytes();
        let mut ans = String::new();
        let mut right = s.len();

        for left in (0..s.len()).rev() {
            if s[left] == b' ' {
                if left + 1 < right {
                    if !ans.is_empty() {
                        ans.push(' ');
                    }
                    ans.push_str(std::str::from_utf8(&s[left + 1..right]).unwrap());
                }
                right = left;
            } else if left == 0 {
                if !ans.is_empty() {
                    ans.push(' ');
                }
                ans.push_str(std::str::from_utf8(&s[left..right]).unwrap());
            }
        }

        ans
    }
}
