import java.util.HashMap;

class Solution {
	public int lengthOfLongestSubstring(String s) {
		Integer max_range_length = 0;
		HashMap<Character, Boolean> map = new HashMap<Character, Boolean>();
		Integer rk = 0; // 滑动窗口右值
		char[] chars = s.toCharArray();
		for (int i = 0; i < chars.length; i++) {
			// 上面for循环会默认滑动左边界，我们排除第一次时不收缩窗口，其他时候都收缩窗口
			if (i != 0) {
				map.remove(chars[i - 1]);
			}
			// 滑动窗口往右界移动
			while (rk < chars.length && !map.containsKey(chars[rk])) {
				map.put(chars[rk], true);
				rk++;
			}
			// 如果滑动窗口大小大于最大值，则更新最大值
			if (rk - i > max_range_length) {
				max_range_length = rk - i;
			}
		}
		return max_range_length;
	}
}