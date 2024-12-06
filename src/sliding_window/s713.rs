use crate::Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut total, mut ans) = (0, 0, 0);
        if k == 0 {
            return 0;
        }

        for right in 0..nums.len() {
            total *= nums[right];

            while total >= k {
                total /= nums[left];
                left += 1;
            }

            ans += right + left + 1;
        }

        ans as i32
    }
}
