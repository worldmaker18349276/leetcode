pub trait Problem3 {
    // Given a string s, find the length of the longest substring without repeating characters.
    fn length_of_longest_substring(s: String) -> i32;
}

struct Solution;

impl Problem3 for Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;

        let bytes = s.as_bytes();
        let mut start = 0;
        let mut length = 0;
        let mut seen = HashSet::new();

        for (i, c) in bytes.iter().enumerate() {
            if seen.contains(c) {
                length = length.max(i - start);
                while &bytes[start] != c {
                    seen.remove(&bytes[start]);
                    start += 1;
                }
                start += 1;
            } else {
                seen.insert(c);
            }
        }

        length = length.max(bytes.len() - start);
        length as i32
    }
}
