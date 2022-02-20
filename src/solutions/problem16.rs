pub trait Problem16 {
    // Given an integer array nums of length n and an integer target, find three
    // integers in nums such that the sum is closest to target. Return the sum
    // of the three integers. You may assume that each input would have exactly
    // one solution.
    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32;
}

struct Solution;

impl Problem16 for Solution {
    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let mut nums = nums;
        nums.sort_unstable();

        let mut res = nums[0] + nums[1] + nums[2];

        for (j, y) in nums.iter().enumerate() {
            let mut forward = nums[..j].iter();
            let mut backward = nums[j+1..].iter().rev();
            let mut left = forward.next();
            let mut right = backward.next();
            while let (Some(x), Some(z)) = (left, right) {
                let sum = x + y + z;
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                }
                match sum.cmp(&target) {
                    Ordering::Less => left = forward.next(),
                    Ordering::Greater => right = backward.next(),
                    Ordering::Equal => return target,
                }
            }
        }

        res
    }
}