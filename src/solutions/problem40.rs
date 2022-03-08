pub trait Problem40 {
    // 1 <= candidates.length <= 100
    // 1 <= candidates[i] <= 50
    // 1 <= target <= 30
    fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem40 for Solution {
    fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();

        fn go(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
            if target == 0 {
                return vec![Vec::new()];
            }
            let mut res = Vec::new();
            let candidates = match candidates.binary_search(&target) {
                Ok(mut i) => {
                    res.push(vec![target]);
                    loop {
                        if candidates[i] != target {
                            break &candidates[..=i];
                        }
                        if i == 0 {
                            break &candidates[..0];
                        }
                        i -= 1;
                    }
                }
                Err(i) => &candidates[..i],
            };
            for (mut i, n) in candidates.iter().enumerate().rev() {
                let mut m = 0;
                let candidates = loop {
                    if candidates[i] != *n {
                        break &candidates[..=i];
                    }
                    m += 1;
                    if i == 0 {
                        break &candidates[..0];
                    }
                    i -= 1;
                };
                if n * m <= target {
                    for mut subres in go(candidates, target - n * m) {
                        for _ in 0..m {
                            subres.push(*n);
                        }
                        res.push(subres);
                    }
                }
            }
            res
        }

        go(&candidates, target)
    }
}
