use crate::Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut i = 0;
        let n = intervals.len();

        // 前序推入
        while i < n && intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
            i += 1;
        }

        // 需要合并的区间做合并处理
        while i < n && intervals[i][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }
        result.push(new_interval);

        // 后序推入
        while i < n {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}
