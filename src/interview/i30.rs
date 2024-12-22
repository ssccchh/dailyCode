use std::collections::HashMap;

use crate::Solution;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let word_count = words.len();
        let total_len = word_len * word_count;
        let mut result = Vec::new();

        // 构建 words 中每个单词的计数哈希表
        let mut word_map = HashMap::new();
        for word in &words {
            *word_map.entry(word.clone()).or_insert(0) += 1;
        }

        // 遍历字符串 s，尝试从每个位置开始检查
        for i in 0..word_len {
            // 从每个词的前缀开始，最多 word_len 次
            let mut left = i; // 窗口左端
            let mut right = i; // 窗口右端
            let mut current_map = HashMap::new(); // 用来存储当前窗口内的单词计数

            while right + word_len <= s.len() {
                // 获取当前窗口的单词
                let word = &s[right..right + word_len];
                right += word_len;

                // 如果当前单词是有效的
                if let Some(&count) = word_map.get(word) {
                    *current_map.entry(word.to_string()).or_insert(0) += 1;

                    // 如果当前单词出现次数超过要求，移动左端
                    while current_map[word] > count {
                        let left_word = &s[left..left + word_len];
                        left += word_len;
                        *current_map.entry(left_word.to_string()).or_insert(0) -= 1;
                    }

                    // 如果窗口内单词数量等于 words 中的单词数量，说明找到了一个有效子串
                    if right - left == total_len {
                        result.push(left as i32);
                    }
                } else {
                    // 如果当前单词不在 words 中，则重置
                    current_map.clear();
                    left = right;
                }
            }
        }

        result
    }
}
