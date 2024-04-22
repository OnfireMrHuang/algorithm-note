
class UnionFind {
	private int[] parent;
	private int[] rank;

	public UnionFind(int n) {
		parent = new int[n];
		rank = new int[n];
		for (int i = 0; i < n; i++) {
			parent[i] = i;
			rank[i] = 1;
		}
	}

	public int find(int x) {
		if (x != parent[x]) {
			parent[x] = find(parent[x]);
		}
		return parent[x];
	}

	public boolean union(int x, int y) {
		int rootX = find(x);
		int rootY = find(y);
		if (rootX == rootY) {
			return false;
		}
		if (rank[rootX] < rank[rootY]) {
			int temp = rootX;
			rootX = rootY;
			rootY = temp;
		}
		parent[rootY] = rootX;
		rank[rootX] += rank[rootY];
		return true;
	}
}

class Solution {
	public int[] findRedundantDirectedConnection(int[][] edges) {
		int[] ans = new int[2];
		// 获取每个节点的入度
		int[] inDegress = getNodeInDegress(edges);
		// 首先判断第一种情况,从后往前遍历,寻找是否存在入度为2的顶点,如果存在,则判断是否为树,如果是树,则返回,否则继续
		for (int i = edges.length - 1; i >= 0; i--) {
			if (inDegress[edges[i][1]] == 2) {
				if (isTree(edges, i)) {
					ans[0] = edges[i][0];
					ans[1] = edges[i][1];
					return ans;
				}
			}
		}
		// 接着判断第二种情况，每个节点的入度都为1，判断是否成环
		UnionFind unionFind = new UnionFind(edges.length + 1);
		for (int[] edge : edges) {
			if (!unionFind.union(edge[0], edge[1])) {
				ans[0] = edge[0];
				ans[1] = edge[1];
				return ans;
			}
		}
		return ans;
	}

	private boolean isTree(int[][] edges, int removeIndex) {
		UnionFind unionFind = new UnionFind(edges.length + 1);
		for (int i = 0; i < edges.length; i++) {
			if (i == removeIndex) {
				continue;
			}
			if (!unionFind.union(edges[i][0], edges[i][1])) {
				return false;
			}
		}
		return true;
	}

	private int[] getNodeInDegress(int[][] edges) {
		int[] inDegrees = new int[edges.length + 1];
		for (int[] edge : edges) {
			inDegrees[edge[1]]++;
		}
		return inDegrees;
	}
}