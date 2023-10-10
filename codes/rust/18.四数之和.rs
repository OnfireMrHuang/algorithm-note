/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        Self::n_sum(&nums[..], 4, target)
    }

    // 定义一个框架用来应对几数之和的场景
    fn n_sum(nums: &[i32], n: i32, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        if nums.is_empty() {
            return res;
        }
        if nums.len() < n as usize {
            return res;
        }
        if n == 2 {
            // 两数之和解法
            let (mut left, mut right) = (0, nums.len() - 1);
            while left < right {
                let sum = nums[left] + nums[right];
                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    let mut tmp = vec![nums[left], nums[right]];
                    res.push(tmp);
                    // 去重
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        } else {
            for i in 0..(nums.len() - n as usize + 1) {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                // 这儿可以进行剪枝，因为nums是提前排序了的，所以:
                // nums[i..i+n]的和一定是最小值，如果target比最小值还小，那么就是没有匹配项，跳过
                // nums[i] + nums[-n + 1..]的和一定是最大值，如果target比这个最大值还大，那么就是没有匹配项，跳过
                let min_sum = Self::sum(&nums[i..i + n as usize]);
                if min_sum > target as i64 {
                    continue;
                }
                let mut last_start = nums.len() - n as usize + 1;
                let max_sum = nums[i] as i64 + Self::sum(&nums[last_start..]);
                if max_sum < target as i64 {
                    continue;
                }
                let sub_res = Self::n_sum(&nums[i + 1..], n - 1, target - nums[i]);
                for mut item in sub_res {
                    item.push(nums[i]);
                    res.push(item);
                }
            }
        }
        res
    }

    fn sum(nums: &[i32]) -> i64 {
        let mut res = 0;
        for num in nums {
            res += num.to_owned() as i64;
        }
        res
    }
}

// @lc code=end
