use crate::Solution;

impl Solution {
    pub fn gcb(x: usize, y: usize) -> usize {
        if y == 0 {
            x
        } else {
            Solution::gcb(y, x % y)
        }
    }
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        let count = Solution::gcb(n, k);

        for index in 0..count {
            let mut current = index;
            let mut prev: i32 = nums[index];
            loop {
                let next_index = (current + k) % n;
                let temp = nums[next_index];
                nums[next_index] = prev;
                prev = temp;
                current = next_index;
                if index == next_index {
                    break;
                }
            }
        }
    }
}
