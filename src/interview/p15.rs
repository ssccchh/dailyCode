use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let ans: Vec<Vec<i32>> = vec![];
        let n = nums.len();
        for (i, num) in nums.iter().enumerate() {
            let j = i + 1;
            let mut k = n - 1;
            while k < j {
                let s = num + nums[j] + nums[k];
                if s > 0 {
                    k -= 1;
                }
            }
        }

        ans
    }
}

