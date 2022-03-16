pub trait Problem45 {
    fn jump(nums: Vec<i32>) -> i32;
}

struct Solution;

impl Problem45 for Solution {
    fn jump(nums: Vec<i32>) -> i32 {
        let mut res = vec![i32::MAX; nums.len()];
        res[0] = 0;
        for (i, n) in nums.into_iter().enumerate() {
            let c = res[i] + 1;
            for r in res.iter_mut().skip(i+1).take(n as usize) {
                if *r > c {
                    *r = c;
                }
            }
        }
        res.pop().unwrap()
    }
}
