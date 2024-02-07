
import java.util.PriorityQueue;

class Solution {
	public int kthSmallest(int[][] matrix, int k) {
		PriorityQueue<Integer> heap = new PriorityQueue<>((n1, n2) -> n2 - n1);
		int rowSize = matrix.length;
		for (int i = 0; i < rowSize; i++) {
			int colSize = matrix[0].length;
			int startIdx = i * colSize;
			for (int j = 0; j < colSize; j++) {
				if (startIdx + j < k) {
					heap.offer(matrix[i][j]);
				} else {
					heap.offer(matrix[i][j]);
					heap.poll();
				}
			}
		}
		if (heap.size() < 1) {
			return 0;
		}
		return heap.peek();
	}
}