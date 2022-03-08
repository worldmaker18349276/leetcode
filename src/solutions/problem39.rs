pub trait Problem39 {
    // 1 <= candidates.length <= 30
    // 1 <= candidates[i] <= 200
    // All elements of candidates are distinct.
    // 1 <= target <= 500
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem39 for Solution {
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();

        fn go(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
            let mut res = Vec::new();
            let candidates = match candidates.binary_search(&target) {
                Ok(i) => {
                    res.push(vec![target]);
                    &candidates[..i]
                }
                Err(i) => &candidates[..i],
            };
            for (i, n) in candidates.iter().enumerate().rev() {
                for mut subres in go(&candidates[..i + 1], target - n) {
                    subres.push(*n);
                    res.push(subres);
                }
            }
            res
        }

        go(&candidates, target)
    }
}
