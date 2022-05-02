pub trait Problem75 {
    fn sort_colors(nums: &mut Vec<i32>);
}

struct Solution;

impl Problem75 for Solution {
    fn sort_colors(nums: &mut Vec<i32>) {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        for &i in nums.iter() {
            match i {
                0 => a += 1,
                1 => b += 1,
                2 => c += 1,
                _ => panic!(),
            }
        }
        for i in nums.iter_mut() {
            match (a, b, c) {
                (0, 0, _) => {
                    *i = 2;
                    c -= 1;
                }
                (0, _, _) => {
                    *i = 1;
                    b -= 1;
                }
                (_, _, _) => {
                    *i = 0;
                    a -= 1;
                }
            }
        }
    }
}
