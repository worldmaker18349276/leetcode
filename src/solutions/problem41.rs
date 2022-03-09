pub trait Problem41 {
    fn first_missing_positive(nums: Vec<i32>) -> i32;
}

struct Solution;

impl Problem41 for Solution {
    fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let range = 0..len as i32;
        nums.into_iter()
            .fold(vec![true; len], |mut table, v| {
                if range.contains(&(v - 1)) {
                    table[(v - 1) as usize] = false;
                }
                table
            })
            .into_iter()
            .enumerate()
            .find(|(_, v)| *v)
            .map_or(len as i32 + 1, |(i, _)| i as i32 + 1)
    }
}
