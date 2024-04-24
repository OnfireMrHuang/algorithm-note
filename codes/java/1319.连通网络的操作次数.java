
class UF {
	private int count;
	private int[] parent;
	private int[] size;

	public UF(int n) {
		count = n;
		parent = new int[n];
		for (int i = 0; i < n; i++) {
			parent[i] = i;
		}
		size = new int[n];
		Arrays.fill(size, 1);
	}

	public int find(int x) {
		while (x != parent[x]) {
			parent[x] = parent[parent[x]];
			x = parent[x];
		}
		return x;
	}

	public boolean connected(int x, int y) {
		int xRoot = find(x);
		int yRoot = find(y);
		return xRoot == yRoot;
	}

	public void union(int x, int y) {
		int xRoot = find(x);
		int yRoot = find(y);
		if (xRoot == yRoot) {
			return;
		}
		if (size[xRoot] > size[yRoot]) {
			parent[yRoot] = xRoot;
			size[xRoot] += size[yRoot];
		} else {
			parent[xRoot] = yRoot;
			size[yRoot] += size[xRoot];
		}
	}

}

class Solution {
	public int makeConnected(int n, int[][] connections) {
		UF uf = new UF(n);
		int idleConnection = 0;
		int unconnected = n - 1;
		for (int[] connected : connections) {
			if (uf.connected(connected[0], connected[1])) {
				idleConnection++;
			} else {
				uf.union(connected[0], connected[1]);
				unconnected--;
			}
		}
		if (idleConnection < unconnected) {
			return -1;
		}
		return unconnected;
	}
}