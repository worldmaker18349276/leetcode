pub trait Problem59 {
    fn generate_matrix(n: i32) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem59 for Solution {
    fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];
        let mut x = 1;
        for i in 0..n/2 {
            for j in 0..(n-2*i-1) {
                res[i][i+j] = x;
                x += 1;
            }
            for j in 0..(n-2*i-1) {
                res[i+j][n-1-i] = x;
                x += 1;
            }
            for j in 0..(n-2*i-1) {
                res[n-1-i][n-1-i-j] = x;
                x += 1;
            }
            for j in 0..(n-2*i-1) {
                res[n-1-i-j][i] = x;
                x += 1;
            }
        }
        if n % 2 == 1 {
            res[n/2][n/2] = x;
        }
        res
    }
}
