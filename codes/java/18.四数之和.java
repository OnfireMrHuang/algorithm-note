
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.List;
import java.util.Arrays;

public class Solution {
	public List<List<Integer>> fourSum(int[] nums, int target) {
		// 首先对数组进行排序
		Arrays.sort(nums);
		return nSum(nums, 0, 4, target);
	}

	List<List<Integer>> nSum(int[] nums, int start, int n, int target) {
		List<List<Integer>> ans = new ArrayList<>();
		if (nums.length < n || n < 2) {
			return ans;
		}
		if (n == 2) {
			int left = start, right = nums.length - 1;
			// 左右双指针相向而行，如果大于target，则右指针左移，如果小于target，则左指针右移
			while (left < right) {
				if (nums[left] + nums[right] > target) {
					right--;
				} else if (nums[left] + nums[right] < target) {
					left++;
				} else {
					List<Integer> tmp = new ArrayList<>();
					tmp.add(nums[left]);
					tmp.add(nums[right]);
					ans.add(tmp);
					// 去重
					while (left < right && nums[left] == nums[left + 1]) {
						left++;
					}
					while (left < right && nums[right] == nums[right - 1]) {
						right--;
					}
					left++;
					right--;
				}
			}
		} else {
			for (int i = start; i < nums.length; i++) {
				// 去重
				if (i > start && nums[i] == nums[i - 1]) {
					continue;
				}
				// 进行剪枝: 因为已经排序过了的，[i..i+n]这n数一定是最小的,如果target比这个还小
				// 说明不可能匹配到结果
				if (i + n <= nums.length && isSkip(nums, i, n, target)) {
					continue;
				}
				// 递归求n-1之和
				List<List<Integer>> tmp = nSum(nums, i + 1, n - 1, target - nums[i]);
				for (List<Integer> t : tmp) {
					t.add(nums[i]);
					ans.add(t);
				}
			}
		}
		return ans;
	}

	boolean isSkip(int[] nums, int start, int n, int target) {
		BigInteger targetInteger = new BigInteger(String.valueOf(target));
		// 这里使用bigInteger来处理，主要解决用例中整数太大溢出导致结果错误的问题
		// 如果target比最小的要小，那么就跳过
		BigInteger minAns = new BigInteger("0");
		for (int i = start; i < start + n; i++) {
			minAns = minAns.add(new BigInteger(String.valueOf(nums[i])));
		}
		if (targetInteger.compareTo(minAns) < 0) {
			return true;
		}
		// 如果target比最大的要大，那么就跳过
		BigInteger maxAns = new BigInteger("0");
		int last_start = nums.length - n;
		for (int i = last_start; i < nums.length; i++) {
			maxAns = maxAns.add(new BigInteger(String.valueOf(nums[i])));
		}
		if (targetInteger.compareTo(maxAns) > 0) {
			return true;
		}
		return false;
	}
}