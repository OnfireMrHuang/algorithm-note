struct UF {
    count: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> Self {
        let mut parent: Vec<usize> = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        Self {
            count: n,
            parent: parent,
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);
        x_root == y_root
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        if self.size[x_root] > self.size[y_root] {
            self.parent[y_root] = x_root;
            self.size[x_root] += self.size[y_root]
        } else {
            self.parent[x_root] = y_root;
            self.size[y_root] += self.size[x_root]
        }
    }
}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut uf = UF::new(n as usize);
        let mut idle_connection = 0;
        let mut unconnected = n - 1;
        for connected in connections {
            if uf.connected(connected[0] as usize, connected[1] as usize) {
                idle_connection += 1;
            } else {
                uf.union(connected[0] as usize, connected[1] as usize);
                unconnected -= 1;
            }
        }
        // 此时如果多余的连接小于未连通的计算机，那么就不能连通
        if idle_connection < unconnected {
            return -1;
        }
        unconnected // 否则返回未连通的计算机数量
    }
}
