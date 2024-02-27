import java.util.PriorityQueue;

class Cell implements Comparable<Cell> {
	int x;
	int y;
	int v;

	public Cell(int x, int y, int v) {
		this.x = x;
		this.y = y;
		this.v = v;
	}

	@Override
	public int compareTo(Cell other) {
		return this.v - other.v;
	}
}

class Solution {
	public int trapRainWater(int[][] heightMap) {
		int m = heightMap.length;
		int n = heightMap[0].length;
		// 如果m或n小于等于2，说明没有桶心，积水为零
		if (m <= 2 || n <= 2) {
			return 0;
		}
		// 定义小根堆
		PriorityQueue<Cell> heap = new PriorityQueue<>();
		// 定义一个访问列表
		boolean[][] visit = new boolean[m][n];
		for (int i = 0; i < m; i++) {
			for (int j = 0; j < n; j++) {
				visit[i][j] = false;
				if (i == 0 || i == m - 1 || j == 0 || j == n - 1) {
					visit[i][j] = true;
					heap.add(new Cell(i, j, heightMap[i][j]));
				}
			}
		}
		int result = 0;
		// 定义遍历的四个方向
		int[] directions = new int[] { -1, 0, 1, 0, -1 };
		while (!heap.isEmpty()) {
			Cell cell = heap.poll();
			// 从该单元格的四个方向查找，
			for (int i = 0; i < 4; i++) {
				int x = cell.x + directions[i];
				int y = cell.y + directions[i + 1];
				// 边界判断
				if (x < 0 || x >= m || y < 0 || y >= n || visit[x][y]) {
					continue;
				}
				// 如果当前单元格的高度小于当前单元格的高度，说明可以积水
				if (cell.v > heightMap[x][y]) {
					result += cell.v - heightMap[x][y];
					// 将当前单元格的高度作为积水的高度
					heightMap[x][y] = cell.v;
				}
				visit[x][y] = true;
				heap.add(new Cell(x, y, heightMap[x][y]));
			}
		}
		return result;
	}
}