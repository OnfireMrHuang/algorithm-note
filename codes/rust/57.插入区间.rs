/*
 * @lc app=leetcode.cn id=57 lang=rust
 *
 * [57] 插入区间
 */

// @lc code=start
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // 取出新区间的开始节点、结束节点
        let mut start_int = new_interval[0];
        let mut end_int = new_interval[1];
        let mut result: Vec<Vec<i32>> = Vec::new();
        // 标识是否已经插入过了
        let mut placed = false;
        for interval in intervals {
            // 如果当前区间的左端点大于插入区间的的右端点，说明插入区间在当前区间的左侧，在没有插入的情况下，插入新区间和当前区间
            if interval[0] > end_int {
                if !placed {
                    result.push(vec![start_int, end_int]);
                    placed = true;
                }
                result.push(interval);
            } else if interval[1] < start_int {
                // 如果当前区间的右端点小于插入区间的左端点，所以插入区间在当前区间的右侧，则直接插入当前区间即可，忽略新区间
                result.push(interval);
            } else {
                // 存在交集，则合并更新插入区间的左端点和右端点
                start_int = std::cmp::min(start_int, interval[0]);
                end_int = std::cmp::max(end_int, interval[1]);
            }
        }
        // 最终如果都没有插入过，则说明是最右侧插入
        if !placed {
            result.push(vec![start_int, end_int]);
        }
        result
    }
}
// @lc code=end
