pub trait Problem31 {
    fn next_permutation(nums: &mut Vec<i32>);
}

struct Solution;

impl Problem31 for Solution {
    fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        for i in (0..len - 1).rev() {
            let a = &&nums[i];
            let maybe_j = nums[i + 1..len]
                .iter()
                .enumerate()
                .find(|(_, b)| a < b)
                .map(|(j, _)| j + i + 1);

            match maybe_j {
                Some(j) => {
                    nums.swap(i, j);
                    return;
                }
                None => {
                    nums[i..].rotate_left(1);
                }
            }
        }
    }
}
