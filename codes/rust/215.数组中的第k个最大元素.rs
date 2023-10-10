/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        // 首先，针对整个nums做一次堆化
        let mut heap_size = nums.len();
        Self::build_max_heap(&mut nums, heap_size);
        for i in (nums.len() - (k as usize) + 1..nums.len()).rev() {
            Self::swap(&mut nums, i, 0);
            heap_size -= 1;
            Self::max_heap_heapify(&mut nums, 0, heap_size);
        }
        nums[0]
    }
    // 构建大顶堆
    fn build_max_heap(nums: &mut Vec<i32>, heap_size: usize) {
        // 从非叶子结点开始堆化，一直堆化到根节点
        for i in (0..nums.len() / 2).rev() {
            Self::max_heap_heapify(nums, i, heap_size)
        }
    }
    fn max_heap_heapify(nums: &mut Vec<i32>, i: usize, heap_size: usize) {
        let l = i * 2 + 1;
        let r = i * 2 + 2;
        let mut largest = i;
        if l < heap_size && nums[l] > nums[largest] {
            largest = l;
        }
        if r < heap_size && nums[r] > nums[largest] {
            largest = r;
        }
        if largest != i {
            Self::swap(nums, i, largest);
            Self::max_heap_heapify(nums, largest, heap_size);
        }
    }

    // 交换元素
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }
}
// @lc code=end
