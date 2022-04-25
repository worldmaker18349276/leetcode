pub trait Problem66 {
    fn plus_one(digits: Vec<i32>) -> Vec<i32>;
}

struct Solution;

impl Problem66 for Solution {
    fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut c = 1;
        for d in digits.iter_mut().rev() {
            *d += c;
            c = 0;
            if *d >= 10 {
                *d -= 10;
                c = 1;
            }
        }
        if c == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}
