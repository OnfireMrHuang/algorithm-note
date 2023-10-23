
import java.util.ArrayList;
import java.util.List;

class Solution {
	public int[][] insert(int[][] intervals, int[] newInterval) {
		int start = newInterval[0];
		int end = newInterval[1];
		List<int[]> res = new ArrayList<>();
		boolean placed = false;
		for (int[] interval : intervals) {
			if (interval[0] > end) {
				if (!placed) {
					res.add(new int[] { start, end });
					placed = true;
				}
				res.add(interval);
			} else if (interval[1] < start) {
				res.add(interval);
			} else {
				start = Math.min(start, interval[0]);
				end = Math.max(end, interval[1]);
			}
		}
		if (!placed) {
			res.add(new int[] { start, end });
		}
		return res.toArray(new int[0][0]);
	}
}
