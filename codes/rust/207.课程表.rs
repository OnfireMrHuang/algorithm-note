impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // 构造邻接矩阵
        let graph = Self::build_graph(num_courses, prerequisites);

        // 定义路径和已经访问过的节点
        let mut path = vec![false; num_courses as usize];
        let mut visited = vec![false; num_courses as usize];

        // 遍历每个课程，如果有一个课程构成环，那么直接返回false
        for course in 0..num_courses {
            let has_ring = Self::detect_has_ring(&graph, course as usize, &mut path, &mut visited);
            if has_ring {
                return false;
            }
        }
        true
    }

    fn detect_has_ring(
        graph: &Vec<Vec<i32>>,
        entry: usize,
        path: &mut Vec<bool>,
        visited: &mut Vec<bool>,
    ) -> bool {
        // 查看当前节点是否已经在路径中了
        if path[entry] {
            return true;
        }

        // 如果不在路径中，则查看是否在其他课程路径中已经访问过，如果访问过，那么直接返回false
        if visited[entry] {
            return false;
        }

        visited[entry] = true;
        path[entry] = true;
        // 递归访问依赖entry的课程
        for &egd in &graph[entry] {
            if Self::detect_has_ring(graph, egd as usize, path, visited) {
                return true;
            }
        }
        path[entry] = false;
        false
    }

    // 返回每个顶点的领接矩阵
    fn build_graph(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 一共有num_courses个顶点
        let mut graph = vec![vec![]; num_courses as usize];
        // 从上游往下游的方式构造图模型
        for prerequisite in prerequisites {
            let from = prerequisite[1] as usize;
            let to = prerequisite[0] as i32;
            graph[from].push(to);
        }
        graph
    }
}
