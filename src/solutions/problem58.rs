pub trait Problem58 {
    fn length_of_last_word(s: String) -> i32;
}

struct Solution;

impl Problem58 for Solution {
    fn length_of_last_word(s: String) -> i32 {
        let mut n = 0;
        let mut spaced = false;
        for c in s.chars() {
            if c == ' ' {
                spaced = true;
            } else if spaced {
                spaced = false;
                n = 1;
            } else {
                n += 1;
            }
        }
        n
    }
}
