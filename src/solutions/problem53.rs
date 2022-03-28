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

struct SolutionDAC;

impl Problem53 for SolutionDAC {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        // (max, left_ended_max, right_ended_max, sum)
        fn go(nums: &[i32]) -> (i32, i32, i32, i32) {
            match nums.len() {
                1 => {
                    let x = nums[0];
                    (x, x, x, x)
                }
                n => {
                    let n = n / 2;
                    let (m1, l1, r1, s1) = go(&nums[..n]);
                    let (m2, l2, r2, s2) = go(&nums[n..]);
                    (m1.max(m2).max(r1 + l2), l1.max(s1 + l2), r2.max(s2 + r1), s1 + s2)
                }
            }
        }
        go(&nums).0
    }
}
