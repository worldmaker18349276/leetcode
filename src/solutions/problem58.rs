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

struct SolutionOneLiner;

impl Problem58 for SolutionOneLiner {
    fn length_of_last_word(s: String) -> i32 {
        s.chars().rev().skip_while(|c| c == &' ').take_while(|c| c != &' ').count() as i32
    }
}
