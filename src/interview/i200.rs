use crate::Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        // 递归处理
        fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if i >= grid.len() || j >= grid[i].len() || grid[i][j] != '1' {
                return;
            }

            grid[i][j] = '2'; // 走过的陆地标记为2
                              // 基于这个陆地，向其它四个方向递归
            dfs(grid, i - 1, j); // 左
            dfs(grid, i + 1, j); // 右
            dfs(grid, i, j - 1); // 下
            dfs(grid, i, j + 1); // 上
        }

        // 遍历矩阵
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    dfs(&mut grid, i, j);
                    ans += 1;
                }
            }
        }

        ans
    }
}
