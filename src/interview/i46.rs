use crate::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];
        let mut used = vec![false; nums.len()];

        fn backtrack(
            nums: &Vec<i32>,
            mut result: &mut Vec<Vec<i32>>,
            mut current: &mut Vec<i32>,
            mut used: &mut Vec<bool>,
        ) {
            if current.len() == nums.len() {
                result.push(current.clone());
                return;
            }

            for i in 0..nums.len() {
                if !used[i] {
                    used[i] = true;
                    current.push(nums[i]);
                    backtrack(&nums, &mut result, &mut current, &mut used);
                    current.pop();
                    used[i] = false;
                }
            }
        }

        backtrack(&nums, &mut result, &mut current, &mut used);

        result
    }
}
