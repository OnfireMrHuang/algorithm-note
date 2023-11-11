class Solution {
	public int leastInterval(char[] tasks, int n) {
		if (tasks.length == 0) {
			return 0;
		}
		int size = tasks.length;
		int[] wordCnt = new int[26];
		for (char task : tasks) {
			int index = task - 'A';
			wordCnt[index] += 1;
		}
		int maxCnt = 1;
		for (int cnt : wordCnt) {
			maxCnt = Math.max(maxCnt, cnt);
		}
		int tot = 0;
		for (int cnt : wordCnt) {
			if (cnt == maxCnt) {
				tot += 1;
			}
		}
		return Math.max(size, (maxCnt - 1) * (n + 1) + tot);
	}
}