/*
 * @lc app=leetcode.cn id=684 lang=rust
 *
 * [684] 冗余连接
 */

// @lc code=start
use std::collections::HashMap;
struct UnionFind {
    parent_map: HashMap<i32, i32>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            parent_map: HashMap::new(),
        }
    }

    pub fn union(&mut self, p: i32, q: i32) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        if p_root == q_root {
            return;
        }
        self.parent_map.insert(q_root, p_root);
    }

    pub fn find(&mut self, x: i32) -> i32 {
        // 如果x没有父节点，那么父节点就是它自己
        if self.parent_map.get(&x).is_none() {
            return x;
        }
        let parent = self.parent_map.get(&x).unwrap().to_owned();
        if parent != x {
            let root = self.find(parent);
            self.parent_map.insert(x, root);
        }
        self.parent_map.get(&x).unwrap().to_owned()
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        // 存储每个节点的父节点
        let mut uf = UnionFind::new();
        for edg in edges {
            let parent = edg.get(0).unwrap().to_owned();
            let children = edg.get(1).unwrap().to_owned();
            if uf.find(parent) == uf.find(children) {
                ans = edg;
            }
            uf.union(parent, children);
        }
        ans
    }
}
// @lc code=end
