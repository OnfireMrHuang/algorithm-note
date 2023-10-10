/*
 * @lc app=leetcode.cn id=834 lang=rust
 *
 * [834] 树中距离之和
 */

// @lc code=start
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dp = vec![0; n as usize]; // dp[i] = sum of distance from i to all nodes
        let mut ans = vec![0; n as usize]; // ans[i] = sum of distance from all nodes to i
        let mut sz = vec![0; n as usize]; // sz[i] = size of subtree rooted at i
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
        // 构造边
        for edg in edges {
            graph[edg[0] as usize].push(edg[1]);
            graph[edg[1] as usize].push(edg[0]);
        }
        // 算第一个节点的距离之和，将dp和sz也保留了下来
        Self::dfs(&graph, &mut dp, &mut sz, 0, -1);
        // 根据第一个以存在的dp和sz，计算出ans
        Self::dfs_adjust(&graph, &mut dp, &mut sz, &mut ans, 0, -1);
        ans
    }

    fn dfs_adjust(
        graph: &Vec<Vec<i32>>,
        dp: &mut Vec<i32>,
        sz: &mut Vec<i32>,
        ans: &mut Vec<i32>,
        u: i32,
        f: i32,
    ) {
        // 将结果写入
        ans[u as usize] = dp[u as usize];
        // 调换父节点和子节点，计算子节点的距离
        for v in graph[u as usize].iter() {
            if *v == f {
                continue;
            }
            let pu = dp[u as usize];
            let pv = dp[*v as usize];
            let su = sz[u as usize];
            let sv = sz[*v as usize];
            dp[u as usize] -= dp[*v as usize] + sz[*v as usize];
            sz[u as usize] -= sz[*v as usize];
            dp[*v as usize] += dp[u as usize] + sz[u as usize];
            sz[*v as usize] += sz[u as usize];
            Self::dfs_adjust(graph, dp, sz, ans, *v, u);
            dp[u as usize] = pu;
            dp[*v as usize] = pv;
            sz[u as usize] = su;
            sz[*v as usize] = sv;
        }
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        dp: &mut Vec<i32>,
        sz: &mut Vec<i32>,
        u: i32,
        f: i32, // 该值用于记录父节点，如果边指回父节点，就不需要再遍历
    ) {
        sz[u as usize] = 1; // 默认子节点数为1，因为自身也是一个节点
        dp[u as usize] = 0; // 初始化数值为0
                            // 遍历u节点的边
        for v in graph[u as usize].iter() {
            if *v == f {
                // 如果边指回父节点，就不需要再遍历
                continue;
            }
            Self::dfs(graph, dp, sz, *v, u);
            sz[u as usize] += sz[*v as usize]; // u节点的子节点数就等于v节点的子节点数加上自身
            dp[u as usize] += dp[*v as usize] + sz[*v as usize]; // u节点的距离就等于v节点的距离加上v节点的子节点数
        }
    }
}
// @lc code=end
