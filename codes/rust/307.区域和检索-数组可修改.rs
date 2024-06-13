
// 首先定义线段树节点
#[derive(Clone, Copy)]
struct SegmentTreeNode {
    left: i32, // 左边界索引
    right: i32, // 右边界索引
    val: i32, // 节点值
}

impl SegmentTreeNode {
    fn new(val: i32) -> Self {
        Self {
            left: -1,
            right: -1,
            val: val
        }
    }
}


struct NumArray {
    size: i32,
    tree: Vec<SegmentTreeNode>, // 线段树
    nums: Vec<i32>, // 原始数组
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let size = nums.len() as i32;
        let mut ans = Self {
            size: size,
            nums: nums,
            tree: vec![SegmentTreeNode::new(0); size as usize * 4]
        };
        ans.build(0, 0, size - 1);
        ans
    }
    
    fn update(&mut self, index: i32, val: i32) {
        if index < 0 {
            return;
        }
        self.nums[index as usize] = val;
        self.update_recursion(index, val, 0, 0, self.size - 1)
    }

    fn update_recursion(&mut self, index: i32, val: i32, node_index: i32, node_left: i32, node_right: i32) {
        // 如果是叶子节点，则直接返回
        if node_left == node_right {
            // 如果刚好更新索引等于叶子索引区间值，那么则更新叶子节点值
            if index == node_left {
                self.tree[node_index as usize].val = val;
            }
            return;
        }
        // 如果是分支节点，则开始计算分支节点
        let mid = node_left + (node_right - node_left) / 2;
        let left_index = node_index * 2 + 1;
        let right_index = node_index * 2 + 2;
        // 判断索引在左子树还是右子树
        if index <= mid {
            self.update_recursion(index, val, left_index, node_left, mid);
        } else {
            self.update_recursion(index, val, right_index, mid + 1, node_right);
        }
        // 更新完成之后，更新父节点的值
        self.tree[node_index as usize].val = self.tree[left_index as usize].val + self.tree[right_index as usize].val;
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum_range_recursion(left, right, 0, 0, self.size - 1)
    }

    fn sum_range_recursion(&self, query_left: i32, query_right: i32, start_index: i32, node_left: i32, node_right: i32) -> i32 {
        // 如果查询区间与节点区间没有交集
        if query_left > node_right || query_right < node_left {
            return 0;
        }
        // 如果查询区间包含了节点区间, 则返回区间值
        if query_left <= node_left && query_right >= node_right {
            return self.tree[start_index as usize].val;
        }
        // 否则区间相交，那么则将区间分为两部分，然后使用查询区间去查询
        let mid = node_left + (node_right - node_left) / 2;
        let left_index = start_index * 2 + 1;
        let right_index = start_index * 2 + 2;

        let left_sum_range = self.sum_range_recursion(query_left, query_right, left_index, node_left, mid);
        let right_sum_range = self.sum_range_recursion(query_left, query_right, right_index, mid + 1, node_right);
        left_sum_range + right_sum_range
    }

    // 开始构建线段树
    fn build(&mut self, index: i32, left: i32, right: i32) {
        // 设置根节点的区间
        self.tree[index as usize].left = left;
        self.tree[index as usize].right = right;
        // 此时，如果left与right的相等，说明区分被划分到只剩一个元素了，那么此时节点的值就是nums[left]
        if left == right {
            self.tree[index as usize].val = self.nums[left as usize];
            return;
        }
        // 如果不想等，则继续划分区间
        let mid = left + (right-left) / 2;
        // 递归构建左子树
        let left_index = index * 2 + 1;
        self.build(left_index, left, mid);
        // 递归构建右子树
        let right_index = index * 2 +2;
        self.build(right_index, mid + 1, right);
        // 然后开始计算节点的值
        self.tree[index as usize].val = self.tree[left_index as usize].val + self.tree[right_index as usize].val;
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */