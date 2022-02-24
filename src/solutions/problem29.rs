pub trait Problem29 {
    // Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod
    // operator. The integer division should truncate toward zero, which means losing its fractional part. For example,
    // 8.345 would be truncated to 8, and -2.7335 would be truncated to -2. Return the quotient after dividing dividend
    // by divisor. Note: Assume we are dealing with an environment that could only store integers within the 32-bit
    // signed integer range: [−2^31, 2^31 − 1]. For this problem, if the quotient is strictly greater than 2^31 - 1,
    // then return 2^31 - 1, and if the quotient is strictly less than -2^31, then return -2^31.
    fn divide(dividend: i32, divisor: i32) -> i32;
}

struct Solution;

impl Problem29 for Solution {
    fn divide(dividend: i32, divisor: i32) -> i32 {
        fn bitlen(mut data: u32) -> i32 {
            for len in 0.. {
                if data == 0 {
                    return len;
                }
                data >>= 1;
            }
            panic!()
        }

        fn go(mut dividend: u32, divisor: u32) -> u32 {
            let shift = bitlen(dividend) - bitlen(divisor);
            let mut div = 0;
            for digit in (0..shift + 1).rev() {
                let val = divisor << digit;
                if val <= dividend {
                    div |= 1 << digit;
                    dividend -= val;
                }
            }
            div
        }

        let div = go(dividend.unsigned_abs(), divisor.unsigned_abs());
        let signum = dividend.signum() * divisor.signum();
        if signum > 0 {
            if div >= i32::MAX as u32 {
                i32::MAX
            } else {
                div as i32
            }
        } else if div >= i32::MIN.unsigned_abs() {
            i32::MIN
        } else {
            -(div as i32)
        }
    }
}
