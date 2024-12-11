use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut ans = 0;
        let s = s.as_bytes();

        let mut reset = false;
        for i in 0..s.len() {
            if s[i] == 32 {
                reset = true;
            } else {
                if reset {
                    ans = 0;
                }
                ans += 1;
                reset = false;
            }
        }

        ans as i32
    }
}
