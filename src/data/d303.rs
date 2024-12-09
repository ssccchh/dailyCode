use crate::Solution;

struct NumArray {
    pre_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut pre_sum = Vec::with_capacity(nums.len() + 1);
        pre_sum.push(0);
        for &num in &nums {
            pre_sum.push(pre_sum.last().unwrap() + num);
        }
        NumArray { pre_sum }
    }

    fn sum_range(&self, left: usize, right: usize) -> i32 {
        self.pre_sum[right + 1] - self.pre_sum[left]
    }
}

impl Solution {
    pub fn run_sum(nums: Vec<i32>) {
        let num_array = NumArray::new(nums);
        num_array.sum_range(3, 4);
    }
}
