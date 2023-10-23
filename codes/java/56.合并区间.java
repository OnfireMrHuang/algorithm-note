
import java.util.LinkedList;
import java.util.Arrays;

class Solution {
    public int[][] merge(int[][] intervals) {
        LinkedList<int[]> res = new LinkedList<>();
        // 按区间的 start 升序排列
        Arrays.sort(intervals, (a, b) -> {
            return a[0] - b[0];
        });

        res.add(intervals[0]);
        for (int i = 1; i < intervals.length; i++) {
            int[] curr = intervals[i];
            // res 中最后一个元素的引用
            int[] last = res.getLast();
            // 如果当前区间的起始值在上一个区间的结束值之前，说明重叠了，
            // 则更新上一个区间的结束值为两个区间的最大结束值
            if (curr[0] <= last[1]) {
                last[1] = Math.max(last[1], curr[1]);
            } else {
                // 否则，说明没有重叠，直接将当前区间加入 res
                res.add(curr);
            }
        }
        return res.toArray(new int[0][0]);
    }
}