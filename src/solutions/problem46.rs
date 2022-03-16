use std::slice::Iter;

pub trait Problem46 {
    fn permute(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem46 for Solution {
    fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn go(res: &mut Vec<Vec<i32>>, perm: Vec<i32>, nums: &[i32], visited: Vec<bool>, length: usize) {
            if length == 0 {
                res.push(perm);
                return;
            }
            for (i, v) in visited.iter().enumerate() {
                if !v {
                    let mut visited = visited.clone();
                    visited[i] = true;
                    let mut perm = perm.clone();
                    perm.push(nums[i]);
                    go(res, perm, nums, visited, length - 1);
                }
            }
        }

        let mut res = Vec::new();
        go(&mut res, vec![], &nums, vec![false; nums.len()], nums.len());
        res
    }
}


// fn go(nums: &Vec<i32>, visited: Vec<bool>, length: usize) -> impl Iterator<Vec<i32>> {
//     if length == 0 {
//         yield vec![];
//         return;
//     }
//     for (i, v) in visited.iter().enumerate() {
//         if !v {
//             let mut visited = visited.clone();
//             visited[i] = true;
//             for mut res in go(nums, visited, length - 1) {
//                 res.insert(0, nums[i]);
//                 yield res;
//             }
//         }
//     }
// }
