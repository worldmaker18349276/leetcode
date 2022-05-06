pub trait Problem80 {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
}

struct Solution;

impl Problem80 for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut shift = 0;
        let mut prev = nums[0];
        let mut count = 0;
        for j in 0..nums.len() {
            if nums[j] != prev {
                count = 0;
                prev = nums[j];
            }
            count += 1;

            if count > 2 {
                shift += 1;
            } else if shift != 0 {
                nums.swap(j, j - shift);
            }
        }
        (nums.len() - shift) as i32
    }
}
