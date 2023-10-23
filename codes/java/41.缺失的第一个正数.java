class Solution {
	public int firstMissingPositive(int[] nums) {
		int size = nums.length;
		for (int i = 0; i < size; i++) {
			// 如果nums[i]在[1, size]的范围内, 并且nums[i] != nums[nums[i] - 1], 则一直交换，直到符合条件
			// 其中while + (nums[i] != nums[nums[i] - 1])的意思是一直循环到将下表为nums[i]-1的元素放到正确的位置
			while (nums[i] > 0 && nums[i] <= size && nums[i] != nums[nums[i] - 1]) {
				int index = nums[i] - 1;
				int temp = nums[i];
				nums[i] = nums[index];
				nums[index] = temp;
			}
		}
		// 遍历数组，如果nums[i] != i + 1, 则返回i + 1
		for (int i = 0; i < size; i++) {
			if (nums[i] != i + 1) {
				return i + 1;
			}
		}
		return size + 1;
	}
}