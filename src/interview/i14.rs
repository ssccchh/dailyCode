use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();

        for str in strs.iter().skip(1) {
            while !str.starts_with(&prefix) {
                if prefix.is_empty() {
                    return prefix;
                }
                prefix.pop();
            }
        }

        prefix
    }
}
