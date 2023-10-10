/*
 * @lc app=leetcode.cn id=220 lang=rust
 *
 * [220] 存在重复元素 III
 */

// @lc code=start
use std::collections::BTreeSet;
use std::collections::HashMap;
impl Solution {
    /*
    该题目我们采用滑动窗口 + 桶排序的方法来解决：
    1、我们需要保持一个大小为index_diff的窗口
    2、我们利用哈希表来模拟一个桶排序，每个桶可以容纳value_diff个元素。
        比如假设桶大小为5，那么1、2、3、4、5都分布在第一个桶里
        6、7、8、9、10都分布在第二个桶里，11、12、13、14、15分布在第三个桶里面。
        现在如果判断的是8是否有i-8 <= 5的值，那么:
        1. 如果第二个桶里面有值，那么就是符合条件
        2. 如果第三个桶里面有值，并且里面的最小值减去8的值小于等于5，那么也是符合条件的
        3. 如果第一个桶里面有值，并且8减去里面的最大值小于等于5，那么也是符合条件的
    3、根据窗口大小，我们把超出窗口大小的元素从哈希桶中删除
     */
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        if value_diff < 0 || index_diff < 0 {
            return false;
        }
        // step1: 建立一个哈希表，用来模拟桶排序
        let mut buckets: HashMap<i32, BTreeSet<i32>> = HashMap::new();
        // step2: 遍历数组
        let mut window_start = 0;
        for (i, num) in nums.iter().enumerate() {
            // 如果窗口大小超过限制，则从对应桶中删除相应元素
            if i - window_start > index_diff as usize {
                let mut start_nth = nums[window_start] / (value_diff + 1);
                // 处理边界情况，当小于0时，默认从-1这个桶开始(不然会和0桶冲突)
                if nums[window_start].to_owned() < 0 {
                    start_nth = (nums[window_start] - value_diff) / (value_diff + 1);
                }
                if let Some(set) = buckets.get_mut(&start_nth) {
                    set.remove(&nums[window_start]);
                }
                window_start += 1;
            }
            // 计算数值属于哪个桶
            let mut nth = num / (value_diff + 1);
            // 处理边界情况，当小于0时，默认从-1这个桶开始(不然会和0桶冲突)
            if num.to_owned() < 0 {
                nth = (num - value_diff) / (value_diff + 1);
            }
            // 1.如果桶里面有值
            if buckets.get(&nth).is_some() && !buckets.get(&nth).unwrap().is_empty() {
                return true;
            }
            // 2.如果相邻的下一个桶有值并且其中最小值和num<= value_diff
            if let Some(set) = buckets.get(&(nth + 1)) {
                if !set.is_empty()
                    && (set.iter().next().unwrap().to_owned() - num).abs() <= value_diff
                {
                    return true;
                }
            }
            // 3. 如果相邻的上一个桶有值并且其中的最大值和num的差值的绝对值<=value_diff
            if let Some(set) = buckets.get(&(nth - 1)) {
                if !set.is_empty()
                    && (set.iter().last().unwrap().to_owned() - num).abs() <= value_diff
                {
                    return true;
                }
            }
            // 将数值插入桶
            let set = buckets.entry(nth).or_insert(BTreeSet::new());
            set.insert(num.to_owned());
        }
        false
    }
}
// @lc code=end
