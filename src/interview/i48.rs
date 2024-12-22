use crate::Solution;

impl Solution {
    pub fn rotate1(matrix: &mut Vec<Vec<i32>>) {
        //         暂存tmp=matrix[i][j]
        // matrix[i][j]←matrix[n−1−j][i]←matrix[n−1−i][n−1−j]←matrix[j][n−1−i]←tmp
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in 0..(n + 1) / 2 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n - 1 - j][i];
                matrix[n - 1 - j][i] = matrix[n - 1 - i][n - 1 - j];
                matrix[n - 1 - i][n - 1 - j] = matrix[j][n - 1 - i];
                matrix[j][n - 1 - i] = tmp;
            }
        }
    }
}
