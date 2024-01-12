
import java.util.HashMap;

class NumArray {

	HashMap<Integer, Integer> preSumMap;

	public NumArray(int[] nums) {
		preSumMap = new HashMap<>();
		int preSum = 0;
		for (int i = 0; i < nums.length; i++) {
			preSum += nums[i];
			preSumMap.put(i, preSum);
		}
	}

	public int sumRange(int left, int right) {
		return preSumMap.getOrDefault(right, 0) - preSumMap.getOrDefault(left - 1, 0);
	}
}
