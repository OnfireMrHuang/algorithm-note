import java.util.HashMap;

class Solution {
	public String minWindow(String s, String t) {
		// 定义两个HashMap，分别维护模式串的字符计数和滑动窗口的字符计数
		HashMap<Character, Integer> need = new HashMap<>();
		HashMap<Character, Integer> window = new HashMap<>();

		// 定义左右窗口指针
		int left = 0;
		int right = 0;

		// 定义最小覆盖子串的起始索引和长度
		int minStart = 0;
		int minEnd = 0;
		int minLength = Integer.MAX_VALUE;

		// 首先构建need HashMap
		for (int i = 0; i < t.length(); i++) {
			char c = t.charAt(i);
			need.put(c, need.getOrDefault(c, 0) + 1);
		}

		int validNum = 0; // 用于判断是否完全覆盖

		// 接着开始滑动窗口
		while (right < s.length()) {
			char c = s.charAt(right);
			right++;

			// 写入到滑动窗口中
			window.put(c, window.getOrDefault(c, 0) + 1);

			// 判断当前字符是否在need哈希表中，如果是则判断是否完全覆盖

		}

	}
}