pub trait Problem55 {
    fn can_jump(nums: Vec<i32>) -> bool;
}

struct Solution;

impl Problem55 for Solution {
    fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut can_reach = vec![false; n];
        can_reach[0] = true;
        for (i, step) in nums.into_iter().enumerate() {
            if can_reach[i] {
                if i + 1 + step as usize >= n {
                    return true;
                }
                for cell in can_reach.iter_mut().skip(i).take(1+step as usize) {
                    *cell = true;
                }
            }
        }
        false
    }
}
