// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.

// You must do it in place.

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        if rows == 0 {
            return;
        }
        let cols = matrix[0].len();

        // Vectors to mark rows and columns to be zeroed
        let mut row_zero = vec![false; rows];
        let mut col_zero = vec![false; cols];

        // Step 1: Identify rows and columns to be zeroed
        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == 0 {
                    row_zero[i] = true;
                    col_zero[j] = true;
                }
            }
        }

        // Step 2: Set the identified rows to zero
        for i in 0..rows {
            if row_zero[i] {
                for j in 0..cols {
                    matrix[i][j] = 0;
                }
            }
        }

        // Step 3: Set the identified columns to zero
        for j in 0..cols {
            if col_zero[j] {
                for i in 0..rows {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
