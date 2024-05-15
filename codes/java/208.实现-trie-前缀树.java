class Trie {

	private Trie[] childrens;
	private boolean isEnd;

	public Trie() {
		this.childrens = new Trie[26];
		this.isEnd = false;
	}

	public void insert(String word) {
		Trie iterator = this;
		char[] chs = word.toCharArray();
		for (int i = 0; i < chs.length; i++) {
			int index = chs[i] - 'a';
			if (iterator.childrens[index] == null) {
				iterator.childrens[index] = new Trie();
			}
			iterator = iterator.childrens[index];
		}
		iterator.isEnd = true;
	}

	public boolean search(String word) {
		Trie iterator = searchPrefix(word);
		return iterator != null && iterator.isEnd;
	}

	public boolean startsWith(String prefix) {
		return searchPrefix(prefix) != null;
	}

	private Trie searchPrefix(String prefix) {
		Trie iterator = this;
		char[] chs = prefix.toCharArray();
		for (int i = 0; i < chs.length; i++) {
			int index = chs[i] - 'a';
			if (iterator.childrens[index] == null) {
				return null;
			}
			iterator = iterator.childrens[index];
		}
		return iterator;
	}
}

/**
 * Your Trie object will be instantiated and called as such:
 * Trie obj = new Trie();
 * obj.insert(word);
 * boolean param_2 = obj.search(word);
 * boolean param_3 = obj.startsWith(prefix);
 */