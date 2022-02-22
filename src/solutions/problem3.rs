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

struct SolutionIter;

impl Problem3 for SolutionIter {
    fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;

        if s.is_empty() {
            return 0;
        }

        let bytes = s.as_bytes();
        let mut start_iter = bytes.iter().enumerate();
        let mut start = start_iter.next().unwrap();
        let mut length = 0;
        let mut seen = HashSet::new();

        for (i, c) in bytes.iter().enumerate() {
            if seen.contains(c) {
                length = length.max(i - start.0);
                while start.1 != c {
                    seen.remove(start.1);
                    start = start_iter.next().unwrap();
                }
                start = start_iter.next().unwrap();
            } else {
                seen.insert(c);
            }
        }

        length = length.max(bytes.len() - start.0);
        length as i32
    }
}
