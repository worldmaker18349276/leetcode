pub trait Problem32 {
    fn longest_valid_parentheses(s: String) -> i32;
}

struct Solution;

impl Problem32 for Solution {
    fn longest_valid_parentheses(s: String) -> i32 {
        let s: Vec<_> = s.chars().map(|c| c == '(').collect();
        let mut shadows: Vec<usize> = Vec::new();
        let mut res = 0;
        let mut prev_is_open = false;
        for (i, &is_open) in s.iter().enumerate() {
            if is_open {
                if prev_is_open || shadows.is_empty() {
                    shadows.push(i);
                }
            } else {
                // is closed
                if !prev_is_open {
                    shadows.pop();
                }
                if let Some(j) = shadows.last() {
                    res = res.max(i - j + 1);
                }
            }
            prev_is_open = is_open;
        }
        res as i32
    }
}
