use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];

        if matrix.is_empty() || matrix[0].is_empty() {
            return result;
        }

        // 维护矩阵四个边界
        let (mut top, mut left) = (0, 0);
        let mut right = matrix[0].len() - 1;
        let mut bottom = matrix.len() - 1;

        while top <= bottom && left <= right {
            // 从左往右处理上边界
            for t in left..=right {
                result.push(matrix[top][t]);
            }

            // 从上往下处理右边界
            for r in top + 1..=bottom {
                result.push(matrix[r][right]);
            }

            // 从右往左处理下边界
            if top < bottom && left < right {
                for b in (left + 1..=right - 1).rev() {
                    result.push(matrix[bottom][b])
                }

                for l in (top + 1..=bottom).rev() {
                    result.push(matrix[l][left])
                }
            }

            // 更新下一个矩阵
            top += 1;
            if right > 0 {
                right -= 1
            };
            if bottom > 0 {
                bottom -= 1
            };
            left += 1;
        }

        result
    }
}
