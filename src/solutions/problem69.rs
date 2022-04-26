pub trait Problem69 {
    fn my_sqrt(x: i32) -> i32;
}

struct Solution;

impl Problem69 for Solution {
    fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        if x == 1 {
            return 1;
        }

        let len = {
            let mut len = 0;
            let mut x_ = x;
            while x_ != 0 {
                x_ /= 2;
                len += 1;
            }
            len
        };

        for y in (1<<(len/2-1)).. {
            let yy = y * y;
            if yy > x || yy < 0 {
                return y-1;
            }
        }

        panic!()
    }
}
