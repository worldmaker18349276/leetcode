pub trait Problem36 {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool;
}

struct Solution;

impl Problem36 for Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let init = 0b1111111111u16;
        fn checker(acc: u16, val: &char) -> u16 {
            if *val == '.' {
                return acc;
            }
            let n = 0b1 << match *val {
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
            if acc & n != 0 {
                acc ^ n
            } else {
                0
            }
        }
        board.iter().all(|row| row.iter().fold(init, checker) != 0)
            && (0..9).all(|i| board.iter().map(|row| &row[i]).fold(init, checker) != 0)
            && board[0..3].iter().flat_map(|row| row[0..3].iter()).fold(init, checker) != 0
            && board[0..3].iter().flat_map(|row| row[3..6].iter()).fold(init, checker) != 0
            && board[0..3].iter().flat_map(|row| row[6..9].iter()).fold(init, checker) != 0
            && board[3..6].iter().flat_map(|row| row[0..3].iter()).fold(init, checker) != 0
            && board[3..6].iter().flat_map(|row| row[3..6].iter()).fold(init, checker) != 0
            && board[3..6].iter().flat_map(|row| row[6..9].iter()).fold(init, checker) != 0
            && board[6..9].iter().flat_map(|row| row[0..3].iter()).fold(init, checker) != 0
            && board[6..9].iter().flat_map(|row| row[3..6].iter()).fold(init, checker) != 0
            && board[6..9].iter().flat_map(|row| row[6..9].iter()).fold(init, checker) != 0
    }
}
