pub trait Problem80 {
    fn search(nums: Vec<i32>, target: i32) -> bool;
}

struct Solution;

impl Problem80 for Solution {
    fn search(nums: Vec<i32>, target: i32) -> bool {
        let left_index = 0;
        let right_index = nums.len() - 1;
        let left = nums[0];
        let right = nums[right_index];

        let mut stack = vec![(left_index, left, right_index, right)];

        if target >= left {
            'left: while let Some((mut left_index, mut left, mut right_index, right)) = stack.pop() {
                while right_index - left_index >= 2 {
                    let mid_index = left_index + (right_index + 1 - left_index) / 2;
                    let mid = nums[mid_index];
                    if mid == left {
                        stack.push((mid_index, mid, right_index, right));
                        right_index = mid_index;
                        // right = mid;
                    } else if mid < left {
                        right_index = mid_index;
                        // right = mid;
                    } else if mid < target {
                        left_index = mid_index;
                        left = mid;
                    } else {
                        if nums[left_index..=mid_index].binary_search(&target).is_ok() {
                            return true;
                        }
                        continue 'left;
                    }
                }
                if left == target || right == target {
                    return true
                }
            }
            false

        } else if target <= right {
            'right: while let Some((mut left_index, left, mut right_index, mut right)) = stack.pop() {
                while right_index - left_index >= 2 {
                    let mid_index = right_index - (right_index + 1 - left_index) / 2;
                    let mid = nums[mid_index];
                    if mid == right {
                        stack.push((left_index, left, mid_index, mid));
                        left_index = mid_index;
                        // left = mid;
                    } else if mid > right {
                        left_index = mid_index;
                        // left = mid;
                    } else if mid > target {
                        right_index = mid_index;
                        right = mid;
                    } else {
                        if nums[mid_index..=right_index].binary_search(&target).is_ok() {
                            return true;
                        }
                        continue 'right;
                    }
                }
                if left == target || right == target {
                    return true
                }
            }
            false

        } else {
            false
        }
    }
}

