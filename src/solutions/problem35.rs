pub trait Problem35 {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32;
}

struct Solution;

impl Problem35 for Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut bot = 0;
        let mut top = nums.len() - 1;

        match target.cmp(&nums[bot]) {
            std::cmp::Ordering::Equal => return bot as i32,
            std::cmp::Ordering::Less => return 0,
            std::cmp::Ordering::Greater => {}
        }

        match target.cmp(&nums[top]) {
            std::cmp::Ordering::Equal => return top as i32,
            std::cmp::Ordering::Greater => return nums.len() as i32,
            std::cmp::Ordering::Less => {}
        }

        while top > bot + 1 {
            let mid = bot + (top - bot) / 2;
            let mid_value = &nums[mid];
            match target.cmp(mid_value) {
                std::cmp::Ordering::Less => top = mid,
                std::cmp::Ordering::Greater => bot = mid,
                std::cmp::Ordering::Equal => return mid as i32,
            }
        }

        top as i32
    }
}
