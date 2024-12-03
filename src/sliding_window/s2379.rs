use crate::Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut change_count = 0;
        let mut ans = k;
        let k = k as usize;
        let block_bytes = blocks.as_bytes();
        for (i, &byte) in block_bytes.iter().enumerate() {
            if byte == b'W' {
                change_count += 1;
            }
            if i < k - 1 {
                continue;
            }
            ans = ans.min(change_count);
            let out = block_bytes[i + 1 - k];
            if out == b'W' {
                change_count -= 1;
            }
        }

        ans
    }
}
