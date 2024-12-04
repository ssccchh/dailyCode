use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (mut left, mut ans) = (0, 0);
        let mut fruit_map = HashMap::new();

        for (i, &fruit) in fruits.iter().enumerate() {
            *fruit_map.entry(fruit).or_insert(0) += 1;

            while fruit_map.len() > 2 {
                let out = fruits[left];
                if let Some(count) = fruit_map.get_mut(&out) {
                    *count -= 1;
                    if *count == 0 {
                        fruit_map.remove(&out);
                    }
                }
                left += 1;
            }

            ans = ans.max(i - left + 1);
        }

        ans as i32
    }
}
