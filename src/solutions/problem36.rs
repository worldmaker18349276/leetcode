pub trait Problem36 {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool;
}

struct Solution;

impl Problem36 for Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let init = 0b1111111111u16;
        fn check(acc: &mut u16, val: &char) -> bool {
            if *val == '.' {
                return true;
            }
            let n = 0b1
                << match *val {
                    '1' => 0,
                    '2' => 1,
                    '3' => 2,
                    '4' => 3,
                    '5' => 4,
                    '6' => 5,
                    '7' => 6,
                    '8' => 7,
                    '9' => 8,
                    _ => panic!(),
                };
            if *acc & n != 0 {
                *acc ^= n;
                true
            } else {
                false
            }
        }
        let mut rows = [init, init, init, init, init, init, init, init, init];
        let mut lines = [init, init, init, init, init, init, init, init, init];
        let mut grids = [init, init, init, init, init, init, init, init, init];
        for (i, row) in board.iter().enumerate() {
            for (j, grid) in row.iter().enumerate() {
                if !check(&mut rows[i], grid) {
                    return false;
                }
                if !check(&mut lines[j], grid) {
                    return false;
                }
                if !check(&mut grids[i / 3 * 3 + j / 3], grid) {
                    return false;
                }
            }
        }
        true
    }
}
