
import java.util.HashMap;

class Solution {
	public int subarraySum(int[] nums, int k) {
		Integer ans = 0;
		HashMap<Integer, Integer> pre_sum_cnt_map = new HashMap<>(); // 前缀和 -> 前缀和出现的次数
		Integer pre_sum = 0; // 上一个数的前缀和
		pre_sum_cnt_map.put(0, 1); // 默认空数组的前缀和为0，出现次数为1
		for (Integer i = 0; i < nums.length; i++) {
			Integer cur_sum = pre_sum + nums[i];
			Integer target = cur_sum - k;
			// 检查target是否存在映射中
			if (pre_sum_cnt_map.containsKey(target)) {
				ans += pre_sum_cnt_map.get(target);
			}
			// 更新pre_sum_cnt_map
			if (pre_sum_cnt_map.containsKey(cur_sum)) {
				pre_sum_cnt_map.put(cur_sum, pre_sum_cnt_map.get(cur_sum) + 1);
			} else {
				pre_sum_cnt_map.put(cur_sum, 1);
			}
			pre_sum = cur_sum;
		}
		return ans;
	}
}
