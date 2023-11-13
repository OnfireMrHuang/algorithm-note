// class Solution {
//     public String longestPalindrome(String s) {
//         String res = "";
//         for (int i = 0; i < s.length(); i++) {
//             // 以 s[i] 为中心的最长回文子串
//             String s1 = palindrome(s, i, i);
//             // 以 s[i] 和 s[i+1] 为中心的最长回文子串
//             String s2 = palindrome(s, i, i + 1);
//             // res = longest(res, s1, s2)
//             res = res.length() > s1.length() ? res : s1;
//             res = res.length() > s2.length() ? res : s2;
//         }
//         return res;
//     }

//     String palindrome(String s, int l, int r) {
//         // 防止索引越界
//         while (l >= 0 && r < s.length()
//                 && s.charAt(l) == s.charAt(r)) {
//             // 向两边展开
//             l--;
//             r++;
//         }
//         // 返回以 s[l] 和 s[r] 为中心的最长回文串
//         return s.substring(l + 1, r);
//     }
// }

/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res = String::new();
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..s.len() {
            // 以 s[i] 为中心的最长回文子串
            let s1 = Solution::palindrome(&s, i, i);
            // 以 s[i] 和 s[i+1] 为中心的最长回文子串
            let s2 = Solution::palindrome(&s, i, i + 1);
            if res.len() > s1.len() {
                res = res;
            } else {
                res = s1;
            }
            if res.len() > s2.len() {
                res = res;
            } else {
                res = s2;
            }
        }
        res
    }

    fn palindrome(s: &Vec<char>, l: usize, r: usize) -> String {
        let mut l = l as i32;
        let mut r = r as i32;
        // 防止索引越界
        while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
            // 向两边展开
            l -= 1;
            r += 1;
        }
        // 返回以 s[l] 和 s[r] 为中心的最长回文串
        s[(l + 1) as usize..r as usize].iter().collect()
    }
}
// @lc code=end
