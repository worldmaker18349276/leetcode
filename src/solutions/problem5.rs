pub trait Problem5 {
    // Given a string s, return the longest palindromic substring in s.
    fn longest_palindrome(s: String) -> String;
}

struct Solution;

impl Problem5 for Solution {
    fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();

        let mut range = 0..=0;
        let mut longest = 1;

        for i in 0..len {
            if len-i <= longest {
                break;
            }
            for j in (i + longest..len).rev() {
                if (0..(j - i + 2) / 2).all(|k| bytes[i + k] == bytes[j - k]) {
                    range = i..=j;
                    longest = j - i + 1;
                    break;
                }
            }
        }

        String::from(&s[range])
    }
}
