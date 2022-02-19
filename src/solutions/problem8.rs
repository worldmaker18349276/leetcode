pub trait Problem8 {
    // Implement the myAtoi(string s) function, which converts a string to a 32-bit signed
    // integer (similar to C/C++'s atoi function).
    // The algorithm for myAtoi(string s) is as follows:
    //     Read in and ignore any leading whitespace.
    //     Check if the next character (if not already at the end of the string) is '-' or
    //     '+'. Read this character in if it is either. This determines if the final result
    //     is negative or positive respectively. Assume the result is positive if neither
    //     is present. Read in next the characters until the next non-digit character or the
    //     end of the input is reached. The rest of the string is ignored. Convert these
    //     digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read,
    //     then the integer is 0. Change the sign as necessary (from step 2). If the integer
    //     is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then clamp the integer
    //     so that it remains in the range. Specifically, integers less than -2^31 should be
    //     clamped to -2^31, and integers greater than 2^31 - 1 should be clamped to 2^31 - 1.
    //     Return the integer as the final result.
    // Note:
    //     Only the space character ' ' is considered a whitespace character. Do not ignore
    //     any characters other than the leading whitespace or the rest of the string after
    //     the digits.
    fn my_atoi(s: String) -> i32;
}

struct Solution;

impl Problem8 for Solution {
    fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars();

        let first = loop {
            match iter.next() {
                Some(' ') => {}
                Some(ch) => break ch,
                None => return 0,
            }
        };

        let (sign, mut num) = match first {
            '+' => (1, iter.next()),
            '-' => (-1, iter.next()),
            ch => (1, Some(ch)),
        };

        let mut value = 0;
        loop {
            let digit = match num {
                Some('0') => 0,
                Some('1') => sign,
                Some('2') => sign * 2,
                Some('3') => sign * 3,
                Some('4') => sign * 4,
                Some('5') => sign * 5,
                Some('6') => sign * 6,
                Some('7') => sign * 7,
                Some('8') => sign * 8,
                Some('9') => sign * 9,
                _ => return value,
            };
            if value < i32::MIN / 10 || sign < 0 && value < (i32::MIN - digit) / 10 {
                return i32::MIN;
            } else if value > i32::MAX / 10 || sign > 0 && value > (i32::MAX - digit) / 10 {
                return i32::MAX;
            } else {
                value = value * 10 + digit;
            }
            num = iter.next();
        }
    }
}

