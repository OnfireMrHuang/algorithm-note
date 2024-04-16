class Solution {
	public int minCostConnectPoints(int[][] points) {
		int n = points.length;
		// 生成所有边及权重
		List<int[]> edges = new ArrayList<>();
		for (int i = 0; i < n; i++) {
			for (int j = i + 1; j < n; j++) {
				int xi = points[i][0], yi = points[i][1];
				int xj = points[j][0], yj = points[j][1];
				// 用坐标点在 points 中的索引表示坐标点
				edges.add(new int[] {
						i, j, Math.abs(xi - xj) + Math.abs(yi - yj)
				});
			}
		}
		// 将边按照权重从小到大排序
		Collections.sort(edges, (a, b) -> {
			return a[2] - b[2];
		});
		// 执行 Kruskal 算法
		int mst = 0;
		UF uf = new UF(n);
		for (int[] edge : edges) {
			int u = edge[0];
			int v = edge[1];
			int weight = edge[2];
			// 若这条边会产生环，则不能加入 mst
			if (uf.connected(u, v)) {
				continue;
			}
			// 若这条边不会产生环，则属于最小生成树
			mst += weight;
			uf.union(u, v);
		}
		return mst;
	}

	class UF {
		// 连通分量个数
		private int count;
		// 存储一棵树
		private int[] parent;
		// 记录树的「重量」
		private int[] size;

		// n 为图中节点的个数
		public UF(int n) {
			this.count = n;
			parent = new int[n];
			size = new int[n];
			for (int i = 0; i < n; i++) {
				parent[i] = i;
				size[i] = 1;
			}
		}

		// 将节点 p 和节点 q 连通
		public void union(int p, int q) {
			int rootP = find(p);
			int rootQ = find(q);
			if (rootP == rootQ)
				return;

			// 小树接到大树下面，较平衡
			if (size[rootP] > size[rootQ]) {
				parent[rootQ] = rootP;
				size[rootP] += size[rootQ];
			} else {
				parent[rootP] = rootQ;
				size[rootQ] += size[rootP];
			}
			// 两个连通分量合并成一个连通分量
			count--;
		}

		// 判断节点 p 和节点 q 是否连通
		public boolean connected(int p, int q) {
			int rootP = find(p);
			int rootQ = find(q);
			return rootP == rootQ;
		}

		// 返回节点 x 的连通分量根节点
		private int find(int x) {
			while (parent[x] != x) {
				// 进行路径压缩
				parent[x] = parent[parent[x]];
				x = parent[x];
			}
			return x;
		}

		// 返回图中的连通分量个数
		public int count() {
			return count;
		}
	}
}