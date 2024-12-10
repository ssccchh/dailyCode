use crate::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut ans, mut min_gas, mut gas_sum) = (0, 0, 0);

        for (i, (&g, c)) in gas.iter().zip(cost).enumerate() {
            gas_sum += g - c; // 计算当前油量
            if gas_sum < min_gas {
                min_gas = gas_sum;
                ans = i + 1;
            }
        }

        if gas_sum < 0 {
            -1
        } else {
            ans as i32
        }
    }
}
