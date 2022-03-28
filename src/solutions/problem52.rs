pub trait Problem52 {
    fn total_n_queens(n: i32) -> i32;
}

struct Solution;

impl Problem52 for Solution {
    fn total_n_queens(n: i32) -> i32 {
        fn go(board: &mut Vec<usize>, res: &mut i32, length: usize) {
            let len = board.len();
            if len == length {
                *res += 1;
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
        let mut res = 0;
        go(&mut board, &mut res, n as usize);

        res
    }
}
