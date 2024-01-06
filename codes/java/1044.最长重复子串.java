import java.util.HashMap;

class Solution {
	public String longestDupSubstring(String s) {
		int l = 0;
		int r = s.length() - 1;
		// 二分查找子串，如果子串是重复子串，那么把长度加大再查找，如果不是重复子串，那么把长度减小再查找
		while (l < r) {
			int m = l + ((r - l + 1) >> 1);
			if (isDuplicatePresent(s, m)) {
				l = m;
			} else {
				r = m - 1;
			}
		}
		return findDuplicate(s, l);
	}

	// 查找末尾为length的重复子串，如果找到了，返回子串，否则返回空串
	private String findDuplicate(String s, int length) {
		long hash = 0; // 子串哈希值
		int prime = 29; // 哈希计算的质数，当每个字符都是小写字母时，取29最好
		long firstEntryPower = 1; // 用于计算子串哈希值的第一项的幂
		HashMap<Long, Integer> map = new HashMap<>();

		char[] sArr = s.toCharArray();

		for (int i = 0; i < length; i++) {
			firstEntryPower *= prime;
			hash = hash * prime + (sArr[i] - 'a');
		}
		map.put(hash, 0);
		for (int i = length; i < s.length(); i++) {
			hash = hash * prime + sArr[i] - 'a';
			hash -= firstEntryPower * (sArr[i - length] - 'a');

			if (map.containsKey(hash)) {
				int idx = map.get(hash);
				return s.substring(idx, idx + length);
			}
			map.put(hash, i - length + 1);
		}
		return "";
	}

	private boolean isDuplicatePresent(String s, int length) {
		if (length == 0) {
			return true;
		}
		return !findDuplicate(s, length).isEmpty();
	}
}