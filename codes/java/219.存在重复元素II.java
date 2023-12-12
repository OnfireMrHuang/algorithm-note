class Solution {
	public boolean containsNearbyDuplicate(int[] nums, int k) {
		Map<Integer, Integer> map = new HashMap<>();
		int len = nums.length;
		for (int i = 0; i < len; i++) {
			if (map.containsKey(nums[i])) {
				if (i - map.get(nums[i]) <= k) {
					return true;
				}
			}
			map.put(nums[i], i);
		}
		return false;
	}
}