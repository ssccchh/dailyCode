use crate::Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col0_flag = false;
        let row = matrix.len();
        let col = matrix[0].len();

        // 第一次遍历矩阵做标记
        for i in 0..row {
            // 判断第一列是否存在0，做标记
            if matrix[i][0] == 0 {
                col0_flag = true;
            }
            for j in 1..col {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // 第二次做0的替换，注意倒序遍历，避免影响已经做好标记的首行首列
        for i in (0..row).rev() {
            for j in (1..col).rev() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // 判断第一列是否为0
        for i in 0..row {
            if col0_flag {
                matrix[i][0] = 0;
            }
        }
    }
}
