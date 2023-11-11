
class Solution {
	public int[] maxSlidingWindow(int[] nums, int k) {
		int length = nums.length;
		int[] result = new int[length - k + 1];
		Deque<Integer> deque = new LinkedList<>();
		for (int i = 0; i < length; i++) {
			if (i < k - 1) {
				while (!deque.isEmpty() && deque.peekLast() < nums[i]) {
					deque.pollLast();
				}
				deque.offerLast(nums[i]);
				continue;
			}
			while (!deque.isEmpty() && deque.peekLast() < nums[i]) {
				deque.pollLast();
			}
			deque.offerLast(nums[i]);
			result[i - k + 1] = deque.peekFirst();
			if (deque.peekFirst() == nums[i - k + 1]) {
				deque.pollFirst();
			}
		}
		return result;
	}
}