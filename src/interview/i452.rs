use crate::Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        // 先按右端点排序区间
        points.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut ans = 1;

        // 第一个点放置在第一个区间的左右面，按贪心算法可以覆盖最多的区间
        let mut pre = points[0][1];

        // 去掉一个区间，遍历剩余区间
        for i in 1..points.len() {
            // 区间起始点比上一个放置点大，那么再放置一个点在当前区间的右端点
            if points[i][0] > pre {
                ans += 1;
                pre = points[i][1]
            }
        }

        ans
    }
}
