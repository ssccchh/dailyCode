use crate::Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        // hash 解法
        // let mut map = HashMap::new();

        // while n != 1 && map.insert(n, 1).is_none() {
        //     n = Solution::get_next(n);
        // }

        // if n == 1 {
        //     true
        // } else {
        //     false
        // }

        // 快慢指针解法
        let mut slow_p = n;
        let mut fast_p = Solution::get_next(n);

        // 快指针会先到达1，或者追上慢指针
        while fast_p != 1 && fast_p != slow_p {
            slow_p = Solution::get_next(slow_p);
            fast_p = Solution::get_next(Solution::get_next(fast_p));
        }

        fast_p == 1
    }

    pub fn get_next(mut n: i32) -> i32 {
        let mut total_sum = 0;
        while n > 0 {
            let d = n % 10;
            n = n / 10;
            total_sum += d * d;
        }
        total_sum
    }
}
