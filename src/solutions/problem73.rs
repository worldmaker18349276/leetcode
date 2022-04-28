pub trait Problem73 {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>);
}

struct Solution;

impl Problem73 for Solution {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = Vec::new();
        let mut columns = Vec::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if c == &0 {
                    if let Err(n) = rows.binary_search(&i) {
                        rows.insert(n, i);
                    }
                    if let Err(n) = columns.binary_search(&j) {
                        columns.insert(n, j);
                    }
                }
            }
        }
        for &i in rows.iter() {
            for j in 0..matrix[0].len() {
                matrix[i][j] = 0;
            }
        }
        for &j in columns.iter() {
            for i in 0..matrix.len() {
                matrix[i][j] = 0;
            }
        }
    }
}

struct SolutionConst;

impl Problem73 for SolutionConst {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let width = matrix[0].len();
        let mut c = false;
        {
            for j in 0..width {
                if matrix[0][j] == 0 {
                    c = true;
                }
            }
        }
        for i in 1..matrix.len() {
            let mut c = false;
            for j in 0..width {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    c = true;
                }
            }
            if c {
                matrix[i].fill(0);
            }
        }
        {
            for j in 0..width {
                if matrix[0][j] == 0 {
                    for line in matrix.iter_mut() {
                        line[j] = 0;
                    }
                }
            }
            if c {
                matrix[0].fill(0);
            }
        }
    }
}
