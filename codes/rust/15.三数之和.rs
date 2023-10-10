/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(param: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = param;
        // 首先对数组进行升序排序
        nums.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let length = nums.len();
        for first in 0..length {
            // 如果当前元素大于0，并且和上一次的元素一样，则跳过，因为已经和第二层、第三层匹配过了
            if first > 0 && nums[first] == nums[first - 1] {
                continue;
            }
            let target = -1 * nums[first];
            let mut third = length - 1;
            for second in first + 1..length {
                // 同样的，如果当前元素大于0，并且和上一次元素一样，则跳过，因为已经匹配过了
                if second > first + 1 && nums[second] == nums[second - 1] {
                    continue;
                }
                while second < third && nums[second] + nums[third] > target {
                    third -= 1;
                }
                // 如果second下标和third下标相等了，说明后面的second或third都不可能再匹配到0了
                if second == third {
                    break;
                }
                // 这是时候second与third有两种情况： 等于0或者小于0，如果等于0，则加入到结果集中
                if nums[second] + nums[third] == target {
                    let mut temp: Vec<i32> = Vec::new();
                    temp.push(nums[first]);
                    temp.push(nums[second]);
                    temp.push(nums[third]);
                    result.push(temp);
                }
            }
        }
        result
    }
}
// @lc code=end
