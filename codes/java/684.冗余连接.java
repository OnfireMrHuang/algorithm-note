
import java.util.HashMap;

class UnionFind {
	private HashMap<Integer, Integer> parent;

	UnionFind() {
		parent = new HashMap<>();
	}

	public Integer find(Integer i) {
		while (parent.get(i) != null) {
			i = parent.get(i);
		}
		return i;
	}

	public void union(Integer p, Integer q) {
		Integer pRoot = find(p);
		Integer qRoot = find(q);
		if (pRoot == qRoot) {
			return;
		}
		parent.put(qRoot, pRoot);
	}
}

class Solution {
	public int[] findRedundantConnection(int[][] edges) {
		UnionFind uf = new UnionFind();
		for (int[] edg : edges) {
			int parent = edg[0];
			int children = edg[1];
			if (uf.find(parent) == uf.find(children)) {
				return edg;
			}
			uf.union(parent, children);
		}
		return new int[0];
	}
}
