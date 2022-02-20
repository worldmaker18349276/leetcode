pub trait Problem26 {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
}

struct Solution;

impl Problem26 for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev = nums[0] - 1;
        let mut index = 0;
        while let Some(value) = nums.get(index) {
            if prev == *value {
                nums.remove(index);
            } else {
                prev = *value;
                index += 1;
            }
        }
        nums.len() as i32
    }
}