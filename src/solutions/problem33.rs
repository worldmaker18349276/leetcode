pub trait Problem33 {
    fn search(nums: Vec<i32>, target: i32) -> i32;
}

struct Solution;

impl Problem33 for Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut bot = 0;
        let mut top = nums.len() - 1;
        let mut bot_value = &nums[bot];
        let mut top_value = &nums[top];

        if bot_value == &target {
            return bot as i32;
        }
        if top_value == &target {
            return top as i32;
        }
        if bot_value < top_value {
            return nums[bot..top].binary_search(&target).map_or(-1, |i| i as i32);
        }

        let is_above = &target > bot_value;
        while top - bot > 1 {
            let mid = bot + (top - bot) / 2;
            let mid_value = &nums[mid];

            if is_above {
                if &target <= mid_value {
                    return nums[bot..=mid].binary_search(&target).map_or(-1, |i| (bot + i) as i32);
                }
            } else {
                // is below
                if mid_value <= &target {
                    return nums[mid..=top].binary_search(&target).map_or(-1, |i| (mid + i) as i32);
                }
            }

            if bot_value < mid_value {
                bot = mid;
                bot_value = mid_value;
            } else {
                top = mid;
                top_value = mid_value;
            }
        }
        -1
    }
}
