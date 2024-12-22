use crate::Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let row = board.len();
        let col = board[0].len();

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for i in 0..row {
            for j in 0..col {
                let mut live_neighbors = 0;
                for &(dx, dy) in &directions {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if x >= 0 && x < row as i32 && y >= 0 && y < col as i32 {
                        // -1 代表将要死亡的细胞，但在当前状态下，算作活细胞
                        if board[x as usize][y as usize] == 1 || board[x as usize][y as usize] == -1
                        {
                            live_neighbors += 1;
                        }
                    }
                }

                // 根据规则，更新状态
                if board[i][j] == 1 && (live_neighbors < 2 || live_neighbors > 3) {
                    board[i][j] = -1; // 活变死
                }

                if board[i][j] == 0 && (live_neighbors == 3) {
                    board[i][j] = 2; // 死变活
                }
            }
        }

        // 中间状态更新
        for i in 0..row {
            for j in 0..col {
                if board[i][j] == -1 {
                    board[i][j] = 0;
                } else if board[i][j] == 2 {
                    board[i][j] = 1;
                }
            }
        }
    }
}
