class Trie {
	Trie[] children;
	boolean isEnd;

	public Trie() {
		this.children = new Trie[26];
		this.isEnd = false;
	}

	public void insert(String word) {
		Trie node = this;
		for (char ch : word.toCharArray()) {
			if (node.children[ch - 'a'] == null) {
				node.children[ch - 'a'] = new Trie();
			}
			node = node.children[ch - 'a'];
		}
		node.isEnd = true;
	}
}

class Solution {
	public String replaceWords(List<String> dictionary, String sentence) {
		// 首先对词典构建字典树
		Trie trie = new Trie();
		for (String root : dictionary) {
			trie.insert(root);
		}

		StringBuilder ans = new StringBuilder();
		// 接着分割sentance，通过空格
		String[] words = sentence.split(" ");
		for (String word : words) {
			// 对每个单词查找前缀
			Trie node = trie;
			boolean find_prefix = false;
			String prefix = "";
			for (char ch : word.toCharArray()) {
				if (node.children[ch - 'a'] != null) {
					prefix += ch;
					if (node.children[ch - 'a'].isEnd) {
						find_prefix = true;
						break;
					}
					node = node.children[ch - 'a'];
				} else {
					break;
				}
			}
			if (find_prefix) {
				ans.append(prefix);
			} else {
				ans.append(word);
			}
			ans.append(" ");
		}
		return ans.toString().trim();
	}
}