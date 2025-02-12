use crate::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];
        fn backtrack(
            mut result: &mut Vec<Vec<i32>>,
            mut current: &mut Vec<i32>,
            start: usize,
            n: usize,
            k: usize,
        ) {
            if current.len() == k {
                result.push(current.clone());
                return;
            }
            for i in start..=n - (k - current.len()) + 1 {
                current.push(i as i32);
                backtrack(&mut result, &mut current, i + 1, n, k);
                current.pop(); // 回溯
            }
        }
        backtrack(&mut result, &mut current, 1, n as usize, k as usize);
        result
    }
}
