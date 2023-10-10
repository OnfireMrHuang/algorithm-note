/*
 * @lc app=leetcode.cn id=685 lang=rust
 *
 * [685] 冗余连接 II
 */

// @lc code=start
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        // 首先遍历整个边，记录每个节点的入度
        let indeges = Self::get_indegrees(&edges);
        //第一种情况: 从后往前遍历，如果某个节点的入度为2，那么就尝试删除其中一条边再判断图是否变成树，如果是，那么就是要删除的边；如果不是，那么就是另一条边
        for i in (0..edges.len()).rev() {
            if indeges[edges[i][1] as usize] == 2 {
                let mut new_edges = edges.clone();
                new_edges.remove(i);
                if Self::is_tree(&new_edges, edges.len()) {
                    return edges[i].clone();
                }
            }
        }
        // 第二种情况: 图中出现了环，这个时候就可以通过并查集来判断，如果某个节点的父节点已经是当前节点的父节点，那么就是要删除的边
        Self::get_remove_edge(&edges, edges.len())
    }

    fn get_indegrees(edges: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegrees = vec![0; edges.len() + 1];
        for edge in edges {
            indegrees[edge[1] as usize] += 1;
        }
        indegrees
    }

    fn is_tree(edges: &Vec<Vec<i32>>, n: usize) -> bool {
        // 并查集
        let mut parent = vec![0; n + 1];
        for i in 0..=n {
            parent[i] = i;
        }

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let pu = Self::find(&parent, u);
            let pv = Self::find(&parent, v);
            if pu == pv {
                return false;
            }
            parent[pv] = pu;
        }
        true
    }

    fn find(parent: &Vec<usize>, i: usize) -> usize {
        if parent[i] == i {
            return i;
        }
        Self::find(parent, parent[i])
    }

    fn get_remove_edge(edges: &Vec<Vec<i32>>, n: usize) -> Vec<i32> {
        let mut parent = vec![0; n + 1];
        for i in 0..=n {
            parent[i] = i;
        }

        let mut remove_edge = vec![];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let pu = Self::find(&parent, u);
            let pv = Self::find(&parent, v);
            if pu == pv {
                remove_edge = edge.clone();
            } else {
                parent[pu] = pv;
            }
        }
        remove_edge
    }
}
// @lc code=end
