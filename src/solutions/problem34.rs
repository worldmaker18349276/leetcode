pub trait Problem34 {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

struct Solution;

impl Problem34 for Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums = {
            let mut tmp = nums;
            tmp.insert(0, i32::MIN);
            tmp.push(i32::MAX);
            tmp
        };

        let mut bot = 0;
        let mut top_ = nums.len() - 1;

        while top_ >= bot + 2 {
            let mid = bot + (top_ - bot) / 2;
            let mid_value = &nums[mid];
            if mid_value < &target {
                bot = mid;
            } else {
                top_ = mid;
            }
        }

        let mut bot_ = 0;
        let mut top = nums.len() - 1;

        while top >= bot_ + 2 {
            let mid = bot_ + (top - bot_) / 2;
            let mid_value = &nums[mid];
            if mid_value > &target {
                top = mid;
            } else {
                bot_ = mid;
            }
        }

        if top >= bot + 2 {
            vec![bot as i32, (top - 2) as i32]
        } else {
            vec![-1, -1]
        }
    }
}
