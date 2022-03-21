pub trait Problem48 {
    fn rotate(matrix: &mut Vec<Vec<i32>>);
}

struct Solution;

impl Problem48 for Solution {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..n - i - 1 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[n - 1 - j][i];
                matrix[n - 1 - j][i] = matrix[n - 1 - i][n - 1 - j];
                matrix[n - 1 - i][n - 1 - j] = matrix[j][n - 1 - i];
                matrix[j][n - 1 - i] = temp;
            }
        }
    }
}

struct SolutionFast;

impl Problem48 for SolutionFast {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..n - i - 1 {
                let i_ = n - 1 - i;
                let j_ = n - 1 - j;
                let a = &matrix[i][j] as *const i32;
                let b = &matrix[j_][i] as *const i32;
                let c = &matrix[i_][j_] as *const i32;
                let d = &matrix[j][i_] as *const i32;
                unsafe {
                    let a = a as *mut i32;
                    let b = b as *mut i32;
                    let c = c as *mut i32;
                    let d = d as *mut i32;
                    let temp = *a;
                    *a = *b;
                    *b = *c;
                    *c = *d;
                    *d = temp;
                }
            }
        }
    }
}
