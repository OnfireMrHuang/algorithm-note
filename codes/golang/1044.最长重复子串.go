

func longestDupSubstring(s string) string {
	l := 0
	r := len(s) - 1
	for l < r {
		m := l + ((r - l + 1) >> 1)
		if isDuplicatePresent(s, m) {
			l = m
		} else {
			r = m - 1
		}
	}
	return findDuplicate(s, l)
}

func findDuplicate(s string, length int) string {
	hash := 0
	prime := 29
	firstEntryPower := 1
	m := make(map[int]int)

	for i := 0; i < length; i++ {
		firstEntryPower *= prime
		hash = hash*prime + int(s[i]-'a')
	}
	m[hash] = 0
	for i := length; i < len(s); i++ {
		hash = hash*prime + int(s[i]-'a')
		hash -= firstEntryPower * int(s[i-length]-'a')
		if _, ok := m[hash]; ok {
			return s[m[hash] : m[hash]+length]
		}
		m[hash] = i - length + 1
	}
	return ""
}

func isDuplicatePresent(s string, length int) bool {
	if length == 0 {
		return true
	}
	return len(findDuplicate(s, length)) != 0
}