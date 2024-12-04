use crate::Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let total = k * 2;

        // 长度不够
        if s.len() < total {
            return false;
        }

        true
    }
}
