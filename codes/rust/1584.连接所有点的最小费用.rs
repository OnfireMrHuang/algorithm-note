struct UF {
    count: i32,       // 并查集数量
    size: Vec<i32>,   // 每个并查集的大小(即顶点数量)
    parent: Vec<i32>, // 记录每个顶点的父节点
}

impl UF {
    fn new(n: i32) -> Self {
        Self {
            count: n,
            size: vec![1; n as usize],
            parent: (0..n).collect(),
        }
    }

    // 返回当前并查集的个数
    fn count(&self) -> i32 {
        self.count
    }

    // 根据顶点查找所属并查集的根节点
    fn find(&mut self, mut x: i32) -> i32 {
        while self.parent[x as usize] != x {
            self.parent[x as usize] = self.parent[self.parent[x as usize] as usize]; // 添加路径压缩
            x = self.parent[x as usize];
        }
        x
    }

    // 判断两个顶点是否连通
    fn connected(&mut self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }

    // 将两个顶点添加到并查集中
    fn union(&mut self, x: i32, y: i32) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        // 如果两个顶点已经连通，则直接返回
        if x_root == y_root {
            return;
        }

        // 将小树的根节点连接到大树的根节点
        if self.size[x_root as usize] < self.size[y_root as usize] {
            self.parent[x_root as usize] = y_root;
            self.size[y_root as usize] += self.size[x_root as usize];
        } else {
            self.parent[y_root as usize] = x_root;
            self.size[x_root as usize] += self.size[y_root as usize];
        }
        self.count -= 1;
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        // 第一步: 生成所有的边和权重
        let n = points.len();
        let mut edgs: Vec<Vec<i32>> = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                let dis = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edgs.push(vec![dis, i as i32, j as i32]);
            }
        }
        // 第二步: 对无向边按照权重大小进行从小到大排列
        edgs.sort();

        // 第三步: 从优先级队列从弹出最小的权重边
        let mut mst = 0;
        let mut uf = UF::new(n as i32);
        for edg in edgs {
            let dis = edg[0];
            let x = edg[1];
            let y = edg[2];
            if !uf.connected(x, y) {
                uf.union(x, y);
                mst += dis;
            }
        }
        mst
    }
}
