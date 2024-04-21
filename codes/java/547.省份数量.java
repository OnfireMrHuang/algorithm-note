
class UnionFind {
	private int[] parent;
	private int count;
	private int[] size;

	public UnionFind(int n) {
		parent = new int[n];
		size = new int[n];
		count = n;
		for (int i = 0; i < n; i++) {
			parent[i] = i;
			size[i] = 1;
		}
	}

	public int find(int p) {
		while (p != parent[p]) {
			parent[p] = parent[parent[p]];
			p = parent[p];
		}
		return p;
	}

	public void union(int p, int q) {
		int rootP = find(p);
		int rootQ = find(q);
		if (rootP == rootQ) {
			return;
		}
		if (size[rootP] > size[rootQ]) {
			parent[rootQ] = rootP;
			size[rootP] += size[rootQ];
		} else {
			parent[rootP] = rootQ;
			size[rootQ] += size[rootP];
		}
		count--;
	}

	public int count() {
		return count;
	}
}

class Solution {
	public int findCircleNum(int[][] isConnected) {
		UnionFind uf = new UnionFind(isConnected.length);
		for (int i = 0; i < isConnected.length; i++) {
			for (int j = i + 1; j < isConnected.length; j++) {
				if (isConnected[i][j] == 1) {
					uf.union(i, j);
				}
			}
		}
		return uf.count();
	}
}