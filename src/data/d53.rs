use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let (mut pre, mut min_pre) = (0, 0);
        for num in nums {
            pre += num;
            ans = ans.max(pre - min_pre);
            min_pre = min_pre.min(pre);
        }
        ans
    }
}
