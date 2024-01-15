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
		for (char c : t.toCharArray()) {
			need.put(c, need.getOrDefault(c, 0) + 1);
		}

		int validNum = 0; // 用于判断是否完全覆盖

		// 接着开始滑动窗口
		char[] sChars = s.toCharArray();
		while (right < sChars.length) {
			char c = sChars[right];
			right++;

			// 写入到滑动窗口中
			window.put(c, window.getOrDefault(c, 0) + 1);

			// 判断当前字符是否在need哈希表中并且need与window中的计数是否相等
			if (need.containsKey(c) && need.get(c).equals(window.get(c))) {
				validNum++;
			}

			// 此时再判断有效值是否等于need的长度，没有则继续扩大右窗口
			if (validNum < need.size()) {
				continue;
			}

			// 现在覆盖了，那么开始收缩左窗口到不能覆盖need为止
			while (left < right && validNum >= need.size()) {
				// 此时对比一下窗口大小与最小覆盖子串的大小
				if (right - left <= minLength) {
					minStart = left;
					minEnd = right;
					minLength = right - left;
				}

				// 接着收缩左窗口
				char leftC = s.charAt(left);
				left++;

				// 接着从window中移去
				window.put(leftC, window.get(leftC) - 1);

				// 判断是否需要减少有效值
				if (need.containsKey(leftC) && need.get(leftC) > window.get(leftC)) {
					validNum--;
				}
			}
		}
		// 最后返回最小覆盖子串
		return minLength == Integer.MAX_VALUE ? "" : s.substring(minStart, minEnd);
	}
}