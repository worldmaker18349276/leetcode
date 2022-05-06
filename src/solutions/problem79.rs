pub trait Problem79 {
    fn exist(board: Vec<Vec<char>>, word: String) -> bool;
}

struct Solution;

impl Problem79 for Solution {
    fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<_> = word.chars().collect();
        let ch0 = word[0];
        let mut init = Vec::new();
        for (i, row) in board.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if ch == &ch0 {
                    init.push((i, j));
                }
            }
        }

        fn go(board: &Vec<Vec<char>>, index: usize, start: (usize, usize), path: &mut Vec<(usize, usize)>, word: &Vec<char>) -> bool {
            if index == word.len() {
                return true;
            }
            let (i, j) = start;
            let ch = word[index];

            if i > 0 && board[i-1][j] == ch && !path.contains(&(i-1, j)) {
                path.push((i-1, j));
                if go(board, index+1, (i-1, j), path, word) {
                    return true;
                }
                path.pop();
            }

            if i < board.len()-1 && board[i+1][j] == ch && !path.contains(&(i+1, j)) {
                path.push((i+1, j));
                if go(board, index+1, (i+1, j), path, word) {
                    return true;
                }
                path.pop();
            }

            if j > 0 && board[i][j-1] == ch && !path.contains(&(i, j-1)) {
                path.push((i, j-1));
                if go(board, index+1, (i, j-1), path, word) {
                    return true;
                }
                path.pop();
            }
            
            if j < board[0].len()-1 && board[i][j+1] == ch && !path.contains(&(i, j+1)) {
                path.push((i, j+1));
                if go(board, index+1, (i, j+1), path, word) {
                    return true;
                }
                path.pop();
            }
            
            false
        }

        for (i, j) in init {
            let mut path = vec![(i, j)];
            if go(&board, 1, (i, j), &mut path, &word) {
                return true;
            }
        }

        false
    }
}
