use crate::Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut vowel_count = 0;
        let mut result = 0;
        let k = k as usize;
        for (i, &b) in s.iter().enumerate() {
            if b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u' {
                vowel_count += 1;
            }
            if i < k - 1 {
                continue;
            }

            result = result.max(vowel_count);
            let j = i + 1 - k;
            if s[j] == b'a' || s[j] == b'e' || s[j] == b'i' || s[j] == b'o' || s[j] == b'u' {
                vowel_count -= 1;
            }
        }
        result
    }
}
