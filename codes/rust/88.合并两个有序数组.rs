impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut p = m + n - 1;
        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[p as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[p as usize] = nums2[j as usize];
                j -= 1;
            }
            p -= 1;
        }
        // 最后如果nums2还有元素，那么直接拷贝到nums1的前面
        if j >= 0 {
            nums1[..=j as usize].copy_from_slice(&nums2[..=j as usize]);
        }
    }
}
