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
        let mut placed = false;
        for interval in intervals {
            // 右侧无交集
            if interval[0] > end_int {
                if !placed {
                    result.push(vec![start_int, end_int]);
                    placed = true;
                }
                result.push(interval);
            } else if interval[1] < start_int {
                // 左侧无交集
                result.push(interval);
            } else {
                start_int = Self::min(start_int, interval[0]);
                end_int = Self::max(end_int, interval[1]);
            }
        }
        if !placed {
            result.push(vec![start_int, end_int]);
        }
        result
    }

    fn max(x: i32, y: i32) -> i32 {
        if x > y {
            return x;
        }
        y
    }

    fn min(x: i32, y: i32) -> i32 {
        if x < y {
            return x;
        }
        y
    }
}
// @lc code=end
