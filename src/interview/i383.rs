use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();
        let r = ransom_note.as_bytes();
        let m = magazine.as_bytes();

        for i in 0..m.len() {
            *map.entry(m[i]).or_insert(0) += 1;
        }

        for i in 0..r.len() {
            if let Some(count) = map.get_mut(&r[i]) {
                *count -= 1;
                if *count < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
