pub trait Problem62 {
    fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32;
}

struct Solution;

impl Problem62 for Solution {
    fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let len = obstacle_grid[0].len();
        let mut p = vec![0; len];
        p[0] = 1;
        let mut x = 0;
        for obstacle_line in obstacle_grid.into_iter() {
            x = 0;
            for (a, obstacle) in p.iter_mut().zip(obstacle_line.into_iter()) {
                if obstacle == 1 {
                    *a = 0;
                    x = 0;
                } else {
                    x += *a;
                    *a = x;
                }
            }
        }
        x
    }
}
