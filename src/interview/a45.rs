use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max_jump = 0;
        for (index, &num) in nums.iter().enumerate() {
            max_jump = max_jump.max(index + num as usize);
            count += 1;
            if max_jump >= nums.len() - 1 {
                return count;
            }
        }

        count
    }
}
