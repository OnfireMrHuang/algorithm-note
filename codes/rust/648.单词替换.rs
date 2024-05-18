#[derive(Clone)]
struct Trie {
    children: Vec<Option<Box<Trie>>>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        let children = vec![None; 26];
        Trie {
            children: children,
            is_end: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut ans = String::new();
        // 首先对字典中的单词进行预处理，构建前缀树
        let mut trie = Trie::new();
        for word in &dictionary {
            trie.insert(word);
        }

        // 接着对sentence句子按空格分割，对每个单词进行查找
        for word in sentence.split_whitespace() {
            let mut node = &trie;
            let mut prefix = String::new();
            let mut find_prefix = false;
            for c in word.chars() {
                let idx = c as usize - 'a' as usize;
                if let Some(next) = &node.children[idx] {
                    prefix.push(c);
                    if next.is_end {
                        find_prefix = true;
                        break;
                    }
                    node = next;
                } else {
                    find_prefix = false;
                    break;
                }
            }
            // 如果找到了前缀，就替换原单词
            if find_prefix {
                ans.push_str(&prefix);
            } else {
                ans.push_str(word);
            }
            ans.push(' ');
        }
        return ans.trim_end().to_string();
    }
}
