/*
 * @lc app=leetcode.cn id=1306 lang=rust
 *
 * [1306] 跳跃游戏 III
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        if arr.is_empty() {
            return false;
        }
        // 超过区间范围
        if start < 0 || start >= arr.len() as i32 {
            return false;
        }
        let mut selected_idx: HashSet<usize> = HashSet::new();
        Self::backtrace(&arr, start as usize, &mut selected_idx)
    }

    fn backtrace(arr: &Vec<i32>, start: usize, selected_idx: &mut HashSet<usize>) -> bool {
        // 如果找到了，则返回true
        if arr[start] == 0 {
            return true;
        }
        // 如果是已经跳过的格子，则返回false
        if selected_idx.contains(&start) {
            return false;
        }
        // 选择向前跳或者向后跳
        let prev_idx = start as i32 - arr[start];
        let next_idx = start as i32 + arr[start];
        selected_idx.insert(start as usize);
        if prev_idx >= 0 {
            if Self::backtrace(arr, prev_idx as usize, selected_idx) {
                return true;
            }
        }
        if next_idx < arr.len() as i32 {
            if Self::backtrace(arr, next_idx as usize, selected_idx) {
                return true;
            }
        }
        false
    }
}
// @lc code=end
