use crate::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let mut rows = vec![String::new(); num_rows as usize];
        let mut row = 0;
        let mut going_down = false;

        for c in s.chars() {
            rows[row].push(c);
            if row == 0 || row == (num_rows - 1) as usize {
                going_down = !going_down;
            }
            row = if going_down { row + 1 } else { row - 1 };
        }

        rows.concat()
    }
}
