pub trait Problem54 {
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32>;
}

struct Solution;

impl Problem54 for Solution {
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        enum Stage {
            TopLeft,
            TopRight,
            BottomRight,
            BottomLeft,
        }
        let mut i_start = 0;
        let mut i_end = (matrix.len()-1) as i32;
        let mut j_start = 0;
        let mut j_end = (matrix[0].len()-1) as i32;
        let mut stage = Stage::TopLeft;
        let mut res = Vec::new();

        while i_start <= i_end && j_start <= j_end {
            match stage {
                Stage::TopLeft => {
                    for j in j_start..=j_end {
                        res.push(matrix[i_start as usize][j as usize]);
                    }
                    i_start += 1;
                    stage = Stage::TopRight;
                }
                Stage::TopRight => {
                    for i in i_start..=i_end {
                        res.push(matrix[i as usize][j_end as usize]);
                    }
                    j_end -= 1;
                    stage = Stage::BottomRight;
                }
                Stage::BottomRight => {
                    for j in (j_start..=j_end).rev() {
                        res.push(matrix[i_end as usize][j as usize]);
                    }
                    i_end -= 1;
                    stage = Stage::BottomLeft;
                }
                Stage::BottomLeft => {
                    for i in (i_start..=i_end).rev() {
                        res.push(matrix[i as usize][j_start as usize]);
                    }
                    j_start += 1;
                    stage = Stage::TopLeft;
                }
            }
        }
        res
    }
}
