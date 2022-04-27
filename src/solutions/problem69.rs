pub trait Problem69 {
    fn my_sqrt(x: i32) -> i32;
}

struct Solution;

impl Problem69 for Solution {
    fn my_sqrt(x: i32) -> i32 {
        // x <= 2^31 - 1
        let m = 46340;
        let mut c = 31;
        let mut y = 0;

        'a: while c > 0 {
            for d in 0..c {
                let r = y | (1<<d);
                let r2 = r * r;
                if r2 > x || r > m {
                    if d == 0 {
                        return y;
                    }
                    c = d - 1;
                    y |= 1<<c;
                    continue 'a;
                }
            }
            c -= 1;
            y |= 1<<c;
        }

        y
    }
}
