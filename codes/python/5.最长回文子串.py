class Solution:
    def longestPalindrome(self, s: str) -> str:
        res = ""
        for i in range(len(s)):
            # 以 s[i] 为中心的最长回文子串
            s1 = self.palindrome(s, i, i)
            # 以 s[i] 和 s[i+1] 为中心的最长回文子串
            s2 = self.palindrome(s, i, i + 1)
            # res = longest(res, s1, s2)
            res = res if len(res) > len(s1) else s1
            res = res if len(res) > len(s2) else s2
        return res

    def palindrome(self, s: str, l: int, r: int) -> str:
        # 防止索引越界
        while l >= 0 and r < len(s) and s[l] == s[r]:
            # 向两边展开
            l -= 1
            r += 1
        # 返回以 s[l] 和 s[r] 为中心的最长回文串
        return s[l+1:r]