/*
 * @lc app=leetcode.cn id=847 lang=rust
 *
 * [847] 访问所有节点的最短路径
 */

// @lc code=start
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        // 初始化队列和映射
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashMap::new();
        for i in 0..graph.len() {
            // 将所有节点放入队列中
            queue.push_back((i, 1 << i, 0));
            visited.insert((i, 1 << i), true);
        }

        // 遍历队列
        while !queue.is_empty() {
            let (u, mask, dist) = queue.pop_front().unwrap();
            if mask == (1 << graph.len()) - 1 {
                return dist;
            }
            for v in graph[u].iter() {
                let v = v.to_owned() as usize;
                let misk_v = mask | (1 << v);
                if visited.get(&(v, 1 << v)).is_some() {
                    visited.insert((v, 1 << v), true);
                    queue.push_back((v, misk_v, dist + 1));
                }
            }
        }
        0
    }
}
// @lc code=end
