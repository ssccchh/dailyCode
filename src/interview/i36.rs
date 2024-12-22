use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = vec![HashSet::new(); 9];
        let mut col = vec![HashSet::new(); 9];
        let mut boxes = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let num = board[i][j];

                if num == '.' {
                    continue;
                }

                // 计算3x3宫格的序列号
                let boxes_index = (i / 3) * 3 + (j / 3);

                if row[i].contains(&num)
                    || col[j].contains(&num)
                    || boxes[boxes_index].contains(&num)
                {
                    return false;
                } else {
                    row[i].insert(num);
                    col[j].insert(num);
                    boxes[boxes_index].insert(num);
                }
            }
        }

        true
    }
}
