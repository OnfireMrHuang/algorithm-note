/*
 * @lc app=leetcode.cn id=1345 lang=rust
 *
 * [1345] 跳跃游戏 IV
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        // 构建映射, 相同值的索引被聚合到一个列表
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, item) in arr.iter().enumerate() {
            let mut list = map.entry(*item).or_insert(Vec::new());
            list.push(i);
        }
        // 定义一个队列，用于广度优先遍历
        let mut depth: i32 = 0;
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(0);
        while !queue.is_empty() {
            // 记录一次队列长度
            let queue_size = queue.len();
            for _ in 0..queue_size {
                let idx = queue.pop_front().unwrap();
                // 如果当前元素是最后一个元素，则退出循环
                if idx == arr.len() - 1 {
                    return depth;
                }
                // 添加前一步
                if idx as i32 - 1 > 0 && !visited.contains(&(idx - 1)) {
                    queue.push_back(idx - 1);
                    visited.insert(idx - 1);
                }
                // 添加后一步
                if idx + 1 < arr.len() && !visited.contains(&(idx + 1)) {
                    queue.push_back(idx + 1);
                    visited.insert(idx + 1);
                }
                // 添加相同值的步子
                if let Some(list) = map.get(&arr[idx]) {
                    for item in list {
                        if visited.contains(item) {
                            continue;
                        }
                        queue.push_back(item.to_owned());
                        visited.insert(item.to_owned());
                    }
                }
                // 添加完队列后删除列表
                map.remove(&arr[idx]);
            }
            depth += 1;
        }
        depth
    }
}
// @lc code=end
