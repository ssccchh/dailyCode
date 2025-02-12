use crate::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut current = vec![];
        let mut candidates = candidates;
        candidates.sort();
        fn backtrack(
            mut result: &mut Vec<Vec<i32>>,
            mut current: &mut Vec<i32>,
            candidates: &Vec<i32>,
            current_sum: i32,
            start: usize,
            target: i32,
        ) {
            if current_sum == target {
                result.push(current.clone());
                return;
            }
            for i in start..candidates.len() {
                if current_sum + candidates[i] > target {
                    break; // 剪枝
                }
                current.push(candidates[i]);
                backtrack(
                    &mut result,
                    &mut current,
                    candidates,
                    current_sum + candidates[i],
                    i,
                    target,
                );
                current.pop();
            }
        }
        backtrack(&mut result, &mut current, &candidates, 0, 0, target);
        result
    }
}
