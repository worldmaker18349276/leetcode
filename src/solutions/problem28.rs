pub trait Problem28 {
    // Implement strStr(). Return the index of the first occurrence of needle in
    // haystack, or -1 if needle is not part of haystack. Clarification: What
    // should we return when needle is an empty string? This is a great question
    // to ask during an interview. For the purpose of this problem, we will
    // return 0 when needle is an empty string. This is consistent to C's
    // strstr() and Java's indexOf().
    fn str_str(haystack: String, needle: String) -> i32;
}

struct Solution;

impl Problem28 for Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_len = haystack.len();
        let needle_len = needle.len();
        if haystack_len < needle_len {
            return -1;
        }
        for index in 0..haystack_len-needle_len+1 {
            if needle[..] == haystack[index..index+needle_len] {
                return index as i32;
            }
        }
        -1
    }
}
