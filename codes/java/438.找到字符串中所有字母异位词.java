
class Solution {
	public List<Integer> findAnagrams(String s, String t) {
		Map<Character, Integer> need = new HashMap<>();
		Map<Character, Integer> window = new HashMap<>();
		for (char c : t.toCharArray())
			need.put(c, need.getOrDefault(c, 0) + 1);

		int left = 0, right = 0;
		int valid = 0;
		List<Integer> res = new ArrayList<>();
		while (right < s.length()) {
			char c = s.charAt(right);
			right++;
			// 进行窗口内数据的一系列更新
			if (need.containsKey(c)) {
				window.put(c, window.getOrDefault(c, 0) + 1);
				if (window.get(c).equals(need.get(c)))
					valid++;
			}
			// 判断左侧窗口是否要收缩
			while (right - left >= t.length()) {
				// 当窗口符合条件时，把起始索引加入 res
				if (valid == need.size())
					res.add(left);
				char d = s.charAt(left);
				left++;
				// 进行窗口内数据的一系列更新
				if (need.containsKey(d)) {
					if (window.get(d).equals(need.get(d)))
						valid--;
					window.put(d, window.get(d) - 1);
				}
			}
		}
		return res;
	}
}