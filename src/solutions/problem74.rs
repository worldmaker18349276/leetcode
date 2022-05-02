pub trait Problem74 {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool;
}

struct Solution;

impl Problem74 for Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.binary_search_by_key(&target, |row| row[0]) {
            Ok(_) => true,
            Err(0) => false,
            Err(i) => matrix[i-1].binary_search(&target).is_ok(),
        }
    }
}
