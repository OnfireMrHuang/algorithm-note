/*
 * @lc app=leetcode.cn id=167 lang=rust
 *
 * [167] 两数之和 II - 输入有序数组
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            if numbers[i] + numbers[j] == target {
                result.push((i + 1) as i32);
                result.push((j + 1) as i32);
                break;
            }
            if numbers[i] + numbers[j] > target {
                j -= 1
            }
            if numbers[i] + numbers[j] < target {
                i += 1
            }
        }
        result
    }
}
// @lc code=end
