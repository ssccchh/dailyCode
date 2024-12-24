use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;

        if height.is_empty() {
            return result;
        }

        let mut left = 0;
        let mut right = height.len() - 1;

        while right >= left {
            let x = (right - left) as i32;
            if height[left] > height[right] {
                result = max(result, height[right] * x);
                right -= 1;
            } else {
                result = max(result, height[left] * x);
                left += 1;
            }
        }
        result
    }
}
