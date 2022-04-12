pub trait Problem62 {
    fn unique_paths(m: i32, n: i32) -> i32;
}

struct Solution;

impl Problem62 for Solution {
    fn unique_paths(m: i32, n: i32) -> i32 {
        // if m == 0 || n == 0 {
        //     1
        // } else {
        //     Self::unique_paths(m - 1, n) + Self::unique_paths(m, n - 1)
        // }

        let (m, n) = if m > n { (m, n) } else { (n, m) };
        let mut p = vec![1; m as usize];
        let mut x = 1;
        for _ in 0..n-1 {
            x = 0;
            for a in p.iter_mut() {
                x += *a;
                *a = x;
            }
        }
        x
    }
}
