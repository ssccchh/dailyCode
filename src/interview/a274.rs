use crate::Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut cnt = vec![0; n + 1];
        for c in citations {
            cnt[n.min(c as usize)] += 1;
        }

        let mut s = 0;
        for (i, &c) in cnt.iter().enumerate().rev() {
            s += c;

            if s >= i {
                return i as i32;
            }
        }

        unreachable!()
    }
}
