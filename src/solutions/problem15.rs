pub trait Problem15 {
    // Given an integer array nums, return all the triplets [nums[i], nums[j],
    // nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] +
    // nums[k] == 0. Notice that the solution set must not contain duplicate
    // triplets.
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem15 for Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        let mut nums = nums;
        nums.sort_unstable();

        let mut res = Vec::new();

        for (j, y) in nums.iter().enumerate() {
            let mut forward = nums[..j].iter();
            let mut backward = nums[j+1..].iter().rev();
            let mut left = forward.next();
            let mut right = backward.next();
            while let (Some(x), Some(z)) = (left, right) {
                match (x + y + z).cmp(&0) {
                    Ordering::Less => left = forward.next(),
                    Ordering::Greater => right = backward.next(),
                    Ordering::Equal => {
                        let triplet = vec![*x, *y, *z];
                        if !res.contains(&triplet) {
                            res.push(triplet);
                        }
                        left = forward.next();
                        right = backward.next();
                    }
                }
            }
        }

        res
    }
}
