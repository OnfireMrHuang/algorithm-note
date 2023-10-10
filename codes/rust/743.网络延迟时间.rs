/*
 * @lc app=leetcode.cn id=743 lang=rust
 *
 * [743] 网络延迟时间
 */

// @lc code=start
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        // 将列表转换为一个二维数组，用横、纵坐标来定位的值表示距离，整个二维数据只有一半有真实值。
        let mut graph = vec![vec![std::i32::MAX; n as usize]; n as usize];
        for time in times {
            graph[(time[0] - 1) as usize][(time[1] - 1) as usize] = time[2];
        }
        //初始化：将源节点s到各个节点的距离d[s][v]初始化为无穷大，将源节点s到自身的距离d[s][s]初始化为0
        let mut dist = vec![std::i32::MAX; n as usize];
        dist[(k - 1) as usize] = 0;
        let mut visited = vec![false; n as usize];
        for _ in 0..n {
            let mut min_dist = std::i32::MAX;
            let mut min_index = 0;
            // 从k节点开始更新最短路径
            for i in 0..n as usize {
                if !visited[i] && dist[i] < min_dist {
                    min_dist = dist[i];
                    min_index = i;
                }
            }
            visited[min_index] = true;
            // 然后再开始访问k的邻接节点,如果d[s][v]+v到w的距离小于d[s][w]，则更新d[s][w]的值
            for i in 0..n as usize {
                if !visited[i] && graph[min_index][i] != std::i32::MAX {
                    dist[i] = dist[i].min(dist[min_index] + graph[min_index][i]);
                }
            }
            //重复步骤2和步骤3，直到所有节点都被访问过。
        }
        let mut result = 0;
        for i in 0..n as usize {
            result = result.max(dist[i]);
        }
        if result == std::i32::MAX {
            -1
        } else {
            result
        }
    }
}
// @lc code=end
