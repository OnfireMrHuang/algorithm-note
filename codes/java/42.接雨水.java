class Solution {
	public int trap(int[] height) {
		if (height.length == 0) {
			return 0;
		}
		int n = height.length;
		int res = 0;
		// 数组充当备忘录
		int[] l_max = new int[n];
		int[] r_max = new int[n];
		// 初始化 base case
		l_max[0] = height[0];
		r_max[n - 1] = height[n - 1];
		// 从左向右计算 l_max
		for (int i = 1; i < n; i++)
			l_max[i] = Math.max(height[i], l_max[i - 1]);
		// 从右向左计算 r_max
		for (int i = n - 2; i >= 0; i--)
			r_max[i] = Math.max(height[i], r_max[i + 1]);
		// 计算答案
		for (int i = 1; i < n - 1; i++)
			res += Math.min(l_max[i], r_max[i]) - height[i];
		return res;
	}
}