impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut res = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            // 如果当前温度大于栈顶温度，那么栈顶温度的结果就可以确定了，于是直接设置结果并弹出栈顶(因为已经确定)
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperatures[i] {
                let idx = stack.pop().unwrap();
                res[idx] = (i - idx) as i32;
            }
            stack.push(i);
        }
        res
    }
}
