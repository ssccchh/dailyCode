use crate::Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut result, mut sub_length): (i32, i32) = (i32::MAX, 0);
        let (mut sum, mut i): (i32, usize) = (0, 0);

        for (pos, val) in nums.iter().enumerate() {
            sum += val;
            while sum >= target {
                sub_length = (pos - i + 1) as i32;
                if result > sub_length {
                    result = sub_length;
                }
                sum -= nums[i];
                i += 1;
            }
        }

        if result == i32::MAX {
            return 0;
        }

        result
    }
}
