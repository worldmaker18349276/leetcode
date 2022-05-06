pub trait Problem78 {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem78 for Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![Vec::new()];
        for c in nums {
            for mut pre in res.clone() {
                pre.push(c);
                res.push(pre);
            }
        }
        res
    }
}
