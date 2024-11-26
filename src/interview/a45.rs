use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut next_jump = 0;
        let mut cur_jump = 0;
        for (index, &num) in nums.iter().enumerate() {
            next_jump = next_jump.max(index + num as usize); // 计算当前可以移动的最大长度
            if index == cur_jump {
                cur_jump = next_jump; // 如果移动到上一段位移的最后一位，进行下一次移动，移动次数+1

                if index != nums.len() - 1 {
                    count += 1;
                }
            }
        }

        count
    }
}
