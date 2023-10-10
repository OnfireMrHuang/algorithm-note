/*
 * @lc app=leetcode.cn id=1340 lang=rust
 *
 * [1340] 跳跃游戏 V
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut max_jump_map: HashMap<usize, i32> = HashMap::new();
        for i in 0..arr.len() {
            Self::recursion(&arr, i, d, &mut max_jump_map);
        }
        let mut max_jump = 0;
        // 遍历哈希映射，其中的最大值就是要返回的结果
        for item in max_jump_map {
            if item.1 > max_jump {
                max_jump = item.1;
            }
        }
        max_jump
    }

    fn recursion(arr: &Vec<i32>, i: usize, d: i32, max_jump_map: &mut HashMap<usize, i32>) {
        // 如果备忘录中已经存在，则直接返回
        if max_jump_map.get(&i).is_some() {
            return;
        }
        // base case, 当前阶梯左右都没得跳跃的,也就是最矮的那个,只能访问自身
        max_jump_map.insert(i, 1);
        // 不存在则开始计算, 首先从右边开始跳
        let mut start_idx = i + 1;
        let mut end_idx = i + d as usize;
        if end_idx > arr.len() - 1 {
            end_idx = arr.len() - 1;
        }
        for j in start_idx..=end_idx {
            // 如果中间出现一个比起跳位置大的，则直接退出，因为不可能跳过去
            if arr[j] >= arr[i] {
                break;
            }
            // 算出来要跳这个阶梯的最大访问下标数
            Self::recursion(arr, j, d, max_jump_map);
            // 然后更新起跳阶梯的最大访问下标数
            let cnt = max_jump_map.get(&j).unwrap().to_owned();
            let jump_cnt = max_jump_map.get_mut(&i).unwrap();
            *jump_cnt = std::cmp::max(*jump_cnt, cnt + 1);
        }

        // 没有左区间
        if i < 1 {
            return;
        }
        // 然后从左区间开始跳
        start_idx = i - 1;
        if i < d as usize {
            end_idx = 0;
        } else {
            end_idx = i - d as usize;
        }
        for j in (end_idx..=start_idx).rev() {
            // 同样的，如果中间的阶梯比起跳阶梯大，则跳不过去
            if arr[j as usize] >= arr[i] {
                break;
            }
            // 计算目标阶梯的跳跃最大访问下标数
            Self::recursion(arr, j as usize, d, max_jump_map);
            // 计算好之后更新值
            let cnt = max_jump_map.get(&(j as usize)).unwrap().to_owned();
            let jump_cnt = max_jump_map.get_mut(&i).unwrap();
            *jump_cnt = std::cmp::max(*jump_cnt, cnt + 1);
        }
    }
}
// @lc code=end
