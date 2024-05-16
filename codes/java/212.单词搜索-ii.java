import java.util.HashSet;
import java.util.List;
import java.util.ArrayList;

class Trie {
	Trie[] children;
	String word;

	public Trie() {
		children = new Trie[26];
		word = null;
	}

	public void insert(String word) {
		Trie node = this;
		for (char c : word.toCharArray()) {
			if (node.children[c - 'a'] == null) {
				node.children[c - 'a'] = new Trie();
			}
			node = node.children[c - 'a'];
		}
		node.word = word;
	}
}

class Solution {

	HashSet<String> resultSet;
	boolean[][] visited;
	char[][] _board;
	final int[][] directions = { { -1, 0 }, { 1, 0 }, { 0, -1 }, { 0, 1 } };

	public List<String> findWords(char[][] board, String[] words) {
		_board = board;
		// 定义一个集合用于存储结果
		resultSet = new HashSet<>();
		// 构建字典树
		Trie root = new Trie();
		for (String word : words) {
			root.insert(word);
		}
		// 定义一个visited数组用于记录是否访问过
		visited = new boolean[_board.length][_board[0].length];

		// 开始遍历字母表
		for (int i = 0; i < _board.length; i++) {
			for (int j = 0; j < _board[0].length; j++) {
				int index = _board[i][j] - 'a';
				// 如果当前字母不在字典树中第一层，那么就直接跳过
				if (root.children[index] == null) {
					continue;
				}
				// 然后标记字母表中该位置为已访问过
				visited[i][j] = true;
				dfs(i, j, root.children[index]);
				visited[i][j] = false;
			}
		}
		List<String> result = new ArrayList<>();
		result.addAll(resultSet);
		return result;
	}

	private void dfs(int i, int j, Trie root) {
		// 首先判断字典树是否匹配上了单词
		if (root.word != null) {
			resultSet.add(root.word);
		}
		// 接着从(i,j)的四个方向开始遍历
		for (int[] direction : directions) {
			int newI = i + direction[0];
			int newJ = j + direction[1];
			// 如果新的位置越界了，那么就直接跳过
			if (newI < 0 || newI >= _board.length || newJ < 0 || newJ >= _board[0].length) {
				continue;
			}
			// 如果新的位置已经访问过了，那么就直接跳过
			if (visited[newI][newJ]) {
				continue;
			}
			// 如果新的位置的字母不在字典树中，那么就直接跳过
			int index = _board[newI][newJ] - 'a';
			if (root.children[index] == null) {
				continue;
			}
			// 然后标记新的位置为已访问过
			visited[newI][newJ] = true;
			// 如果新的位置在字典树中，那么就继续深度优先搜索
			dfs(newI, newJ, root.children[index]);
			// 然后标记新的位置为未访问过
			visited[newI][newJ] = false;
		}
	}
}