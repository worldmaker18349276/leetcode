pub trait Problem9 {
    // Given an integer x, return true if x is palindrome integer.
    // An integer is a palindrome when it reads the same backward as forward.
    // For example, 121 is a palindrome while 123 is not.
    fn is_palindrome(x: i32) -> bool;
}

struct Solution;

impl Problem9 for Solution {
    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut x = x;
        let mut digit = Vec::new();
        while x != 0 {
            digit.push(x % 10);
            x /= 10;
        }
        digit.iter().zip(digit.iter().rev()).all(|(x, y)| x == y)
    }
}