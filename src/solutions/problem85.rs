pub trait Problem85 {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32;
}

struct Solution;

impl Problem85 for Solution {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let mut max = 0;
        let mut dp: Vec<Vec<(usize, usize)>> = vec![vec![]; matrix[0].len()];

        for (i, row) in matrix.into_iter().enumerate() {
            let mut filled = 0;
            for ((j, ch), dpj) in row.into_iter().enumerate().zip(dp.iter_mut()) {
                if ch == '1' {
                    let ym = j - filled;
                    match dpj.binary_search_by_key(&std::cmp::Reverse(ym), |(_, y)| std::cmp::Reverse(*y)) {
                        Ok(n) => {
                            max = max.max(dpj.drain(n+1..).map(|(x, y)| (i - x) * (j - y + 1)).max().unwrap_or(0));
                        }
                        Err(n) => {
                            if let Some(xy) = dpj.get_mut(n) {
                                max = max.max((i - xy.0) * (j - xy.1 + 1));
                                xy.1 = ym;
                                max = max.max(dpj.drain(n+1..).map(|(x, y)| (i - x) * (j - y + 1)).max().unwrap_or(0));
                            } else {
                                dpj.push((i, ym));
                            }
                        }
                    }
                    filled += 1;
                } else {
                    max = max.max(dpj.drain(..).map(|(x, y)| (i - x) * (j - y + 1)).max().unwrap_or(0));
                    filled = 0;
                }
                
            }
        }
        for (j, dpj) in dp.into_iter().enumerate() {
            max = max.max(dpj.into_iter().map(|(x, y)| (n - x) * (j - y + 1)).max().unwrap_or(0));
        }

        max as i32
    }
}
