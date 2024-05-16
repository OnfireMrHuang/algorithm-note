use std::collections::HashSet;

#[derive(Clone)]
struct Trie {
    children: Vec<Option<Box<Trie>>>,
    s: String,
}

impl Trie {
    fn new() -> Self {
        let children = vec![None; 26];
        Trie {
            children,
            s: "".to_string(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            let index = c as u8 - 'a' as u8;
            if node.children[index as usize].is_none() {
                node.children[index as usize] = Some(Box::new(Trie::new()));
            }
            node = node.children[index as usize].as_mut().unwrap();
        }
        node.s = word;
    }
}

// 定义四个方向的数组
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        // 定义结果集
        let mut res: HashSet<String> = HashSet::new();
        // 首先，针对words构建字典树
        let mut trie = Trie::new();
        for word in words.iter() {
            trie.insert(word.clone());
        }
        // 定义是否已访问过二维数组
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        // 遍历二维数组
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let first = board[i][j] as u8 - 'a' as u8;
                if trie.children[first as usize].is_none() {
                    continue;
                }
                visited[i][j] = true;
                Self::dfs(
                    &board,
                    trie.children[first as usize].as_ref().unwrap(),
                    &mut visited,
                    i as i32,
                    j as i32,
                    &mut res,
                );
                visited[i][j] = false;
            }
        }
        res.into_iter().collect()
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        trie: &Trie,
        visited: &mut Vec<Vec<bool>>,
        i: i32,
        j: i32,
        res: &mut HashSet<String>,
    ) {
        if trie.s != "" {
            res.insert(trie.s.clone());
        }
        for dir in DIRECTIONS.iter() {
            let new_i = i + dir.0;
            let new_j = j + dir.1;
            if new_i < 0
                || new_i >= board.len() as i32
                || new_j < 0
                || new_j >= board[0].len() as i32
            {
                continue;
            }
            let c = board[new_i as usize][new_j as usize] as u8 - 'a' as u8;
            if visited[new_i as usize][new_j as usize] || trie.children[c as usize].is_none() {
                continue;
            }
            visited[new_i as usize][new_j as usize] = true;
            Self::dfs(
                board,
                trie.children[c as usize].as_ref().unwrap(),
                visited,
                new_i,
                new_j,
                res,
            );
            visited[new_i as usize][new_j as usize] = false;
        }
    }
}
