
class Solution {
	public int[] nextGreaterElements(int[] nums) {
		int[] result = new int[nums.length * 2];
		Arrays.fill(result, -1);
		Stack<Integer> stack = new Stack<>();
		int[] doubleNums = new int[nums.length * 2];
		for (int i = 0; i < doubleNums.length; i++) {
			doubleNums[i] = nums[i % nums.length];
		}
		for (int i = 0; i < doubleNums.length; i++) {
			while (!stack.isEmpty() && doubleNums[stack.peek()] < doubleNums[i]) {
				int lastIdx = stack.pop();
				result[lastIdx] = doubleNums[i];
			}
			stack.push(i);
		}
		int[] res = new int[nums.length];
		for (int i = 0; i < nums.length; i++) {
			res[i] = result[i];
		}
		return res;
	}
}