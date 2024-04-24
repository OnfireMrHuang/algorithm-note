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
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut uf = UF::new(26);
        // 首先合并是==的等式
        equations.iter().for_each(|equation| {
            let equation_chars: Vec<char> = equation.chars().collect();
            let left = equation_chars[0] as usize - 'a' as usize;
            let right = equation_chars[3] as usize - 'a' as usize;
            if equation_chars[1] == '=' {
                uf.union(left, right);
            }
        });
        // 此时并查集中的元素都是相等的，此时判断非等式如果出现在并查集中，说明冲突了，返回false
        for equation in equations.iter() {
            let equation_chars: Vec<char> = equation.chars().collect();
            let left = equation_chars[0] as usize - 'a' as usize;
            let right = equation_chars[3] as usize - 'a' as usize;
            if equation_chars[1] == '!' {
                if uf.connected(left, right) {
                    return false;
                }
            }
        }
        true
    }
}
