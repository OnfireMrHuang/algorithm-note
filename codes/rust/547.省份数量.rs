struct UnionFind {
    n: usize,           // 省份数量
    size: Vec<usize>,   // 每个省份的城市数量
    parent: Vec<usize>, // 每个城市关联的省会城市(根节点)
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut size = vec![1; n];
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        Self { n, size, parent }
    }

    // 返回省份数量
    fn count(&self) -> usize {
        self.n
    }

    // 查找省会城市
    fn find(&mut self, mut i: usize) -> usize {
        while self.parent[i] != i {
            self.parent[i] = self.parent[self.parent[i]]; //(使用隔代压缩)
            i = self.parent[i];
        }
        i
    }

    // 合并城市
    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i == root_j {
            return;
        }
        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }
        self.n -= 1;
    }

    // 判断两个城市是否属于同一个省份
    fn connected(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        // 初始化并查集(假设每个城市一个单独的省份)
        let mut uf = UnionFind::new(n);
        // 遍历城市之间的连通性矩阵，对城市进行省份合并
        for i in 0..n {
            for j in i + 1..n {
                if is_connected[i][j] == 1 {
                    uf.union(i, j);
                }
            }
        }
        uf.count() as i32
    }
}
