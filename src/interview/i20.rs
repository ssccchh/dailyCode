use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // 奇数直接返回false
        if (s.len() % 2) != 0 {
            return false;
        }
        let mut stack = vec![];
        let s = s.as_bytes();
        for i in 0..s.len() {
            // 分支匹配
            match s[i] {
                b'(' => stack.push(b')'),
                b'{' => stack.push(b'}'),
                b'[' => stack.push(b']'),
                _ => {
                    // 匹配右括号出栈
                    if stack.pop() != Some(s[i]) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
