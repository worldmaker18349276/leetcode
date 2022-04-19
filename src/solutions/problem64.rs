pub trait Problem64 {
    fn min_path_sum(grid: Vec<Vec<i32>>) -> i32;
}

struct Solution;

impl Problem64 for Solution {
    fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let len = grid[0].len();
        let mut p = vec![i32::MAX; len];
        p[0] = 0;
        let mut b = i32::MAX;
        for line in grid.into_iter() {
            b = i32::MAX;
            for (a, weight) in p.iter_mut().zip(line.into_iter()) {
                b = b.min(*a) + weight;
                *a = b;
            }
        }
        b
    }
}
