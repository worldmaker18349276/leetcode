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
        let s0 = &strs[0];
        for i in 0..len {
            if strs.iter().any(|s| s[i..i+1] != s0[i..i+1]) {
                return s0[..i].to_string();
            }
        }
        s0[..len].to_string()
    }
}