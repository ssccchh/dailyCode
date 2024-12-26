use crate::Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut i = 0;
        let n = nums.len();
        let mut result = vec![];

        while i < n {
            let start = i;
            i += 1;

            // 连续子区间循环
            while i < n && nums[i] - nums[i - 1] == 1 {
                i += 1;
            }

            // 找终点下标
            let end = i - 1;

            // 两种情况，终点值大于起点值，和终点值等于起点值
            if nums[end] == nums[start] {
                result.push(nums[start].to_string());
            } else {
                result.push(format!("{}->{}", nums[start], nums[end]));
            }
        }

        result
    }
}
