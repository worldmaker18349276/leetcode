pub trait Problem1 {
    // Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    // You may assume that each input would have exactly one solution, and you may not use the same element twice.
    // You can return the answer in any order.
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

struct Solution;

impl Problem1 for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len {
            for j in i + 1..len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!("no answer!");
    }
}

struct SolutionFast;

impl Problem1 for SolutionFast {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut required = HashMap::<i32, usize>::new();
        for (i, value) in nums.iter().enumerate() {
            match required.get(value) {
                Some(j) => {
                    return vec![i as i32, *j as i32];
                }
                None => {
                    required.insert(target - value, i);
                }
            }
        }
        panic!("no answer!");
    }
}
