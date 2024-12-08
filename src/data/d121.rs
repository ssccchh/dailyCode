use std::i32::MAX;

use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut profit, mut min_price) = (0, MAX);
        for price in prices {
            min_price = min_price.min(price);
            profit = profit.max(price - min_price);
        }
        profit
    }
}
