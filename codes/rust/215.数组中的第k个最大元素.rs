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
        // 然后交换堆底和堆首，之后向下调整
        for i in (nums.len() - (k as usize) + 1..nums.len()).rev() {
            Self::swap(&mut nums, i, 0);
            heap_size -= 1;
            Self::max_heap_heapify(&mut nums, 0, heap_size);
        }
        nums[0]
    }
    // 构建大顶堆
    fn build_max_heap(nums: &mut Vec<i32>, heap_size: usize) {
        // 在二叉堆中，数组的下标从0开始，对于任意一个节点i，
        // 它的左子节点的下标为2i+1，右子节点的下标为2i+2。
        // 而对于一个具有n个节点的完全二叉树，最后一个非叶子节点的下标为n/2向下取整-1。
        // 因此，二叉堆的非叶子节点数量为n/2向下取整。
        // 从最后一个非叶子节点开始，不断调整，直到根节点
        for i in (0..nums.len() / 2).rev() {
            Self::max_heap_heapify(nums, i, heap_size)
        }
    }

    // 最大堆向下调整
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
