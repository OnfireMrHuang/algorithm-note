/*
 * @lc app=leetcode.cn id=210 lang=rust
 *
 * [210] 课程表 II
 */

// @lc code=start

use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        // 存储有向图
        let mut edges: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
        // 存储每个节点的入度
        let mut indeg: Vec<i32> = vec![0; num_courses as usize];
        // 存储答案
        let mut result: Vec<i32> = Vec::new();

        // 遍历节点
        for info in prerequisites.iter() {
            // 写入边
            edges[info[1] as usize].push(info[0]);
            // 写节点的入度
            indeg[info[0] as usize] += 1;
        }

        // 定义一个队列，用于广度优先遍历
        let mut queue: VecDeque<i32> = VecDeque::new();
        // 将所有入度为0的节点放入到队列中
        for i in 0..num_courses {
            if indeg[i as usize] == 0 {
                queue.push_back(i);
            }
        }
        // 从队列中取元素
        while !queue.is_empty() {
            // 从队首捞出来一个数据
            let u = queue.pop_front().unwrap();
            // 放入到答案中
            result.push(u);
            // 取u的邻边顶点
            for v in edges[u as usize].iter() {
                let index = v.to_owned();
                // 将邻边节点的入度减一
                indeg[index as usize] -= 1;
                // 如果此时v的入度为0了，那么就把该节点加入到队列中
                if indeg[index as usize] == 0 {
                    queue.push_back(index);
                }
            }
        }
        // 如果结果的数量不等于课程数，那么就说明存在环，直接返回空数组
        if result.len() != num_courses as usize {
            return vec![];
        }
        result
    }
}
// @lc code=end
