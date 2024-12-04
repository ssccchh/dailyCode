use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0; // 左指针
        let mut ans = 0;
        let s = s.as_bytes();
        let mut window = vec![false; 128]; // s为字符、空格、数字，最多有128种

        for (right, &byte) in s.iter().enumerate() {
            let b = byte as usize;
            // 存在，左指针移动
            while window[b] {
                window[s[left] as usize] = false;
                left += 1; // 右移移除存在的元素
            }
            window[b] = true;
            ans = ans.max(right - left + 1)
        }

        ans as i32
    }
}
