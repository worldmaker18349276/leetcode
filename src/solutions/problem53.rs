pub trait Problem53 {
    fn max_sub_array(nums: Vec<i32>) -> i32;
}

struct Solution;

impl Problem53 for Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut m: i32 = i32::MIN;
        let mut n: i32 = 0;
        for x in nums.into_iter() {
            if n < 0 {
                n = 0;
            }
            n += x;
            if n > m {
                m = n;
            }
        }
        m
    }
}
