pub trait Problem14 {
    // Write a function to find the longest common prefix string amongst an
    // array of strings. If there is no common prefix, return an empty string
    // "".
    fn longest_common_prefix(strs: Vec<String>) -> String;
}

struct Solution;

impl Problem14 for Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        let len = strs.iter().map(String::len).min().unwrap();
        let strs: Vec<Vec<_>> = strs.iter().map(|s| s.chars().collect()).collect();
        let str0 = &strs[0][..len];
        let strs = &strs[1..];
        for (i, c0) in str0.iter().enumerate() {
            if strs.iter().any(|s| &s[i] != c0) {
                return str0[..i].iter().collect();
            }
        }
        str0.iter().collect()
    }
}