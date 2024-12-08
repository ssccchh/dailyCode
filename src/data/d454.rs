use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut map = HashMap::new();

        for &num1 in &nums1 {
            for &num2 in &nums2 {
                *map.entry(num1 + num2).or_insert(0) += 1;
            }
        }

        let mut count = 0;
        for &num3 in &nums3 {
            for &num4 in &nums4 {
                if let Some(&c) = map.get(&(0 - (num3 + num4))) {
                    count += c;
                }
            }
        }

        count
    }
}
