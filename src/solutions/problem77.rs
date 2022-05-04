pub trait Problem77 {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem77 for Solution {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn go(i: i32, n: i32, k: i32, pre: &[i32], res: &mut Vec<Vec<i32>>) {
            for j in i..=n {
                let mut pre_ = pre.to_owned();
                pre_.push(j);
                if k == 1 {
                    res.push(pre_);
                } else {
                    go(j+1, n, k-1, &pre_, res);
                }
            }
        }
        let mut res = Vec::new();
        let pre = Vec::new();
        go(1, n, k, &pre, &mut res);
        res
    }
}
