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
            if len - i <= longest {
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

pub struct ManacherAlgorithm;

impl Problem5 for ManacherAlgorithm {
    fn longest_palindrome(s: String) -> String {
        let mut chars = Vec::new();
        for c in s.chars() {
            chars.push('#');
            chars.push(c);
        }
        chars.push('#');
        let len = chars.len();

        let mut res = 0;

        let mut p = vec![0; len];
        let mut c = 0;
        let mut r = 0;

        // 0 <= 2*c-r <= c <= i <= r
        // p[c] == r - c

        for i in 0..len {
            if i > r {
                c = i;
                r = i;
            }
            let i_ = 2 * c - i;
            match p[i_].cmp(&(r - i)) {
                std::cmp::Ordering::Less => p[i] = p[i_],
                std::cmp::Ordering::Greater => p[i] = r - i,
                std::cmp::Ordering::Equal => {
                    p[i] = r - i;
                    let mut r_ = 2 * i - r;
                    while r < len - 1 && r_ > 0 && chars[r_ - 1] == chars[r + 1] {
                        p[i] += 1;
                        r_ -= 1;
                        r += 1;
                    }
                    c = i;
                    if p[i] > p[res] {
                        res = i;
                    }
                }
            }
        }

        chars[res - p[res]..=res + p[res]]
            .iter()
            .filter(|ch| ch != &&'#')
            .collect()
    }
}
