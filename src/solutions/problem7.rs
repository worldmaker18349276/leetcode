pub trait Problem7 {
    // Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes
    // the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
    // Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
    fn reverse(x: i32) -> i32;
}

struct Solution;

impl Problem7 for Solution {
    fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut y = 0;
        while x != 0 {
            if y < i32::MIN / 10
                || y > i32::MAX / 10
                || x < 0 && y < (i32::MIN - x % 10) / 10
                || x > 0 && y > (i32::MAX - x % 10) / 10
            {
                return 0;
            }
            y = y * 10 + x % 10;
            x /= 10;
        }
        y
    }
}
