pub trait Problem37 {
    fn solve_sudoku(board: &mut Vec<Vec<char>>);
}

struct Solution;

impl Problem37 for Solution {
    fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        struct SudokuSolver {
            rows: [u16; 9],
            lines: [u16; 9],
            grids: [u16; 9],
        }

        impl SudokuSolver {
            fn new() -> Self {
                SudokuSolver {
                    rows: [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    lines: [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    grids: [0, 0, 0, 0, 0, 0, 0, 0, 0],
                }
            }

            fn check(&mut self, i: usize, j: usize, grid: &char) -> bool {
                fn check_slot(acc: &mut u16, val: &char) -> bool {
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
                    let res = *acc & n == 0;
                    *acc ^= n;
                    res
                }
                let mut res = true;
                res &= check_slot(&mut self.rows[i], grid);
                res &= check_slot(&mut self.lines[j], grid);
                res &= check_slot(&mut self.grids[i / 3 * 3 + j / 3], grid);
                res
            }

            fn solve(&mut self, board: &mut Vec<Vec<char>>) -> bool {
                fn solve_rec(
                    this: &mut SudokuSolver,
                    board: &mut Vec<Vec<char>>,
                    mut ij: usize,
                ) -> bool {
                    static NUMBERS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
                    while ij < 81 {
                        let i = ij / 9;
                        let j = ij % 9;
                        if board[i][j] == '.' {
                            for n in NUMBERS.iter() {
                                board[i][j] = *n;
                                if this.check(i, j, n) && solve_rec(this, board, ij + 1) {
                                    return true;
                                }
                                this.check(i, j, n);
                            }
                            board[i][j] = '.';
                            return false;
                        }
                        ij += 1;
                    }
                    true
                }

                for (i, row) in board.iter().enumerate() {
                    for (j, grid) in row.iter().enumerate() {
                        if !self.check(i, j, grid) {
                            return false;
                        }
                    }
                }

                solve_rec(self, board, 0)
            }
        }

        assert!(SudokuSolver::new().solve(board));
    }
}
