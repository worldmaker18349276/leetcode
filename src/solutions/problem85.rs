pub trait Problem85 {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32;
}

struct Solution;

impl Problem85 for Solution {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut max = 0;
        let mut dp: Vec<Vec<(usize, usize)>> = vec![vec![]; matrix[0].len()];

        for (i, row) in matrix.into_iter().enumerate() {
            let mut filled = 0;
            for (j, ch) in row.into_iter().enumerate() {
                if ch == '0' {
                    let y = j - filled;
                    match dp[j].binary_search_by_key(&std::cmp::Reverse(y), |(_, y)| std::cmp::Reverse(*y)) {
                        Ok(n) => {
                            dp[j].drain(n+1..);
                        }
                        Err(n) => {
                            if let Some(dpn) = dp[j].get_mut(n) {
                                dpn.1 = y;
                                dp[j].drain(n+1..);
                            } else {
                                dp[j].push((i, y));
                            }
                        }
                    }
                    filled += 1;
                } else {
                    max = max.max(dp[j].drain(..).map(|(x, y)| (i - x + 1) * (j - y + 1)).max().unwrap_or(0));
                    filled = 0;
                }
                
            }
        }

        max as i32
    }
}
