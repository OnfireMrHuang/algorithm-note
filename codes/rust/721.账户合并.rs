/*
 * @lc app=leetcode.cn id=721 lang=rust
 *
 * [721] 账户合并
 */

// @lc code=start

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (x_root, y_root) = (self.find(x), self.find(y));
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else if self.rank[x_root] > self.rank[y_root] {
            self.parent[y_root] = x_root;
        } else {
            self.parent[y_root] = x_root;
            self.rank[x_root] += 1;
        }
    }
}
use std::{
    collections::{HashMap, HashSet},
    vec,
};
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = Vec::new();
        // 定义一个哈希表，用于存储邮箱和对应的账户索引
        let mut email_to_idx: HashMap<String, usize> = HashMap::new();
        let n = accounts.len();
        // 创建并查集
        let mut uf = UnionFind::new(n);
        // 遍历账户，如果邮箱已经存在，则将当前账户合并到该集合中
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&j) = email_to_idx.get(email) {
                    uf.union(i, j);
                }
                email_to_idx.insert(email.clone(), i);
            }
        }

        // 现在我们有了一个账户的合并集合，一个邮箱到账户的哈希表
        // 我们遍历哈希表，取出来所有的邮箱，然后根据邮箱映射的账户去并查集中查找根账户
        // 如果是同一个集合，说明这个这些邮箱都属于这个根账户，我们将这些邮箱放到一个集合中
        let mut idx_to_emails: HashMap<usize, Vec<String>> = HashMap::new();
        for (k, v) in email_to_idx {
            let root_user_idx = uf.find(v);
            idx_to_emails
                .entry(root_user_idx)
                .or_insert(Vec::new())
                .push(k);
        }
        // 这一段我们就有了账户->邮箱列表的映射，接下来再去重排序就好了
        for (k, v) in idx_to_emails {
            let account = (&accounts[k][0]).to_owned();
            let mut emails = v;
            emails.sort();
            let mut item = vec![account];
            item.extend(emails);
            ans.push(item);
        }
        ans
    }
}
// @lc code=end
