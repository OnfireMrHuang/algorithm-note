/*
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 */

// @lc code=start
#[derive(Clone)]
struct Trie {
    children: Vec<Option<Trie>>,
    is_end: bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            children: vec![None; 26],
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut iterator = self;
        // 遍历字母
        for c in word.chars().into_iter() {
            // 获取该字母的索引
            let idx = (c as u8 - 'a' as u8) as usize;
            // 如果该字母对应的子节点不存在，则创建一个
            if iterator.children[idx].is_none() {
                iterator.children.insert(idx, Some(Trie::new()));
            }
            iterator = iterator.children[idx].as_mut().unwrap();
        }
        // 标记该单词已经结束
        iterator.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let res = Self::search_prefix(&self, word);
        if res.is_some() && res.unwrap().is_end {
            return true;
        }
        false
    }

    fn starts_with(&self, prefix: String) -> bool {
        Self::search_prefix(&self, prefix).is_some()
    }

    fn search_prefix(&self, prefix: String) -> Option<&Trie> {
        let mut iterator = self;
        for c in prefix.chars().into_iter() {
            // 获取当前字符的下标
            let idx = (c as u8 - 'a' as u8) as usize;
            if iterator.children[idx].is_none() {
                return None;
            }
            iterator = iterator.children[idx].as_ref().unwrap();
        }
        Some(iterator)
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

