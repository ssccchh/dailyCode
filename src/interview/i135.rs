use crate::Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n == 0 {
            return 0;
        }

        let mut candy = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candy[i] = candy[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candy[i] = candy[i].max(candy[i + 1] + 1);
            }
        }

        candy.iter().sum()
    }
}
