import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Solution {
	public List<List<Integer>> threeSum(int[] nums) {
		List<List<Integer>> ans = new ArrayList<>();
		Arrays.sort(nums);
		for (int first = 0; first < nums.length; first++) {
			// 判断第一个数是否重复
			if (first > 0 && nums[first] == nums[first - 1]) {
				continue;
			}
			// 设置后两个数的目标值
			int target = -nums[first];
			// 开始寻找第二个值
			for (int second = first + 1; second < nums.length; second++) {
				// 判断第二个数是否重复
				if (second > first + 1 && nums[second] == nums[second - 1]) {
					continue;
				}
				// 接下来使用双指针寻找第三个数
				int third = nums.length - 1;
				while (second < third && nums[second] + nums[third] > target) {
					third--;
				}

				// 此时再判断第二个数和第三个数是否相等，如果相等说明没有符合的答案了
				if (second == third) {
					break;
				}

				// 此时如果符合条件,那么就将答案加入到结果集中
				if (nums[second] + nums[third] == target) {
					List<Integer> list = new ArrayList<>();
					list.add(nums[first]);
					list.add(nums[second]);
					list.add(nums[third]);
					ans.add(list);
				}
			}
		}
		return ans;
	}
}