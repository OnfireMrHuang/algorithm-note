use std::collections::HashMap;

struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut sums = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                sums[i + 1][j + 1] = sums[i][j + 1] + sums[i + 1][j] - sums[i][j] + matrix[i][j];
            }
        }
        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sums[row2 as usize + 1][col2 as usize + 1]
            - self.sums[row1 as usize][col2 as usize + 1]
            - self.sums[row2 as usize + 1][col1 as usize]
            + self.sums[row1 as usize][col1 as usize]
    }
}
