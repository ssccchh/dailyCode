use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        // 数字到字母的映射
        let digit_to_letters: [&str; 10] = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];

        let mut result = vec![];
        let mut current = vec![];

        let digits: Vec<char> = digits.chars().collect();

        fn backtrack(
            digits: &[char],
            digit_to_letters: &[&str; 10],
            mut result: &mut Vec<String>,
            mut current: &mut Vec<char>,
            index: usize,
        ) {
            if current.len() == digits.len() {
                result.push(current.iter().collect());
                return;
            }

            // 获取当前数字对应的字母
            let current_digits = digits[index].to_digit(10).unwrap() as usize;
            let letters = digit_to_letters[current_digits];

            for letter in letters.chars() {
                current.push(letter);
                // 递归处理下一个数字
                backtrack(
                    &digits,
                    &digit_to_letters,
                    &mut result,
                    &mut current,
                    index + 1,
                );

                // 撤销选择（回溯）
                current.pop();
            }
        }

        backtrack(&digits, &digit_to_letters, &mut result, &mut current, 0);

        result
    }
}
