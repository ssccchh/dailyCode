use crate::Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut word_len = 0;
        let mut result: Vec<String> = vec![];
        let mut sub_string = String::new();
        for word in words {
            word_len += word.len();
            if word_len as i32 > max_width {
                sub_string.clear();
            } else {
                sub_string.push_str(&word);
            }
        }

        result
    }
}
