pub trait Problem51 {
    fn solve_n_queens(n: i32) -> Vec<Vec<String>>;
}

struct Solution;

impl Problem51 for Solution {
    fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn go(board: &mut Vec<usize>, res: &mut Vec<Vec<usize>>, length: usize) {
            let len = board.len();
            if len == length {
                res.push(board.clone());
                return;
            }

            let mut valid = vec![true; length];
            for (i, &j) in board.iter().rev().enumerate() {
                valid[j] = false;

                if j > i {
                    valid[j - i - 1] = false;
                }

                if j + i + 1 < length {
                    valid[j + i + 1] = false;
                }
            }

            for (i, is_valid) in valid.into_iter().enumerate() {
                if !is_valid {
                    continue;
                }
                board.push(i);
                go(board, res, length);
                board.pop();
            }
        }

        let mut board = Vec::new();
        let mut res = Vec::new();
        go(&mut board, &mut res, n as usize);

        res.into_iter()
            .map(|board| {
                board
                    .into_iter()
                    .map(|i| {
                        let mut row = vec!["."; n as usize];
                        row[i] = "Q";
                        row.into_iter().collect::<String>()
                    })
                    .collect()
            })
            .collect()
    }
}
