class Solution {
	public int[] dailyTemperatures(int[] temperatures) {
		int[] res = new int[temperatures.length];
		Stack<Integer> stack = new Stack<>();
		for (int i = 0; i < temperatures.length; i++) {
			while (!stack.isEmpty() && temperatures[i] > temperatures[stack.peek()]) {
				int preIndex = stack.pop();
				res[preIndex] = i - preIndex;
			}
			stack.push(i);
		}
		return res;
	}
}