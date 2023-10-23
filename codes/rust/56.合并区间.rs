/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start
impl Solution {
    pub fn merge(vals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 1.首先对区间按照区间起始值进行排序
        let mut vals = vals;
        vals.sort_by(|a, b| a[0].cmp(&b[0]));

        // 2. 初始化: 将第一个区间添加到结果中
        let mut result: Vec<Vec<i32>> = Vec::new();
        result.push(vals[0].clone());

        // 3. 遍历区间，将区间进行合并
        for i in 1..vals.len() {
            let interval = vals[i].clone();
            let last_interval = result.last_mut().unwrap();
            // 如果当前区间的结束点 > 上一个区间的结束点，说明不重叠，将当前区间添加到结果中
            if interval[0] > last_interval[1] {
                result.push(interval);
                continue;
            }
            // 否则说明重叠，将上一个区间的结束点更新为和当前区间结束点的最大值
            last_interval[1] = std::cmp::max(last_interval[1], interval[1]);
        }
        result
    }
}
// @lc code=end
