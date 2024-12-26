use crate::Solution;

impl Solution {
    pub fn merge2(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 先按区间起点排序
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            if let Some(last) = result.last_mut() {
                if interval[0] <= last[1] {
                    last[1] = last[1].max(interval[1])
                } else {
                    // 添加新区间
                    result.push(interval);
                }
            } else {
                // 添加第一个区间
                result.push(interval);
            }
        }

        result
    }
}
