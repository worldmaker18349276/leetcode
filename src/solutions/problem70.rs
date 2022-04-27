pub trait Problem70 {
    fn climb_stairs(n: i32) -> i32;
}

struct Solution;

impl Problem70 for Solution {
    fn climb_stairs(n: i32) -> i32 {
        let mut this = 1;
        let mut next = 0;
        for _ in 0..n {
            let temp = next + this;
            next = this;
            this = temp;
        }
        this
    }
}
