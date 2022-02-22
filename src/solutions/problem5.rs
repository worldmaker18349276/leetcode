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

struct SolutionIter;

impl Problem5 for SolutionIter {
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
                let forward = bytes[i..(j + i + 2) / 2].iter();
                let backward = bytes[(j + i) / 2..j + 1].iter().rev();
                if forward.zip(backward).all(|(a, b)| a == b) {
                    range = i..=j;
                    longest = j - i + 1;
                    break;
                }
            }
        }

        String::from(&s[range])
    }
}

struct ManacherAlgorithm;

impl Problem5 for ManacherAlgorithm {
    fn longest_palindrome(s: String) -> String {
        let mut chars = Vec::new();
        for c in s.chars() {
            chars.push('#');
            chars.push(c);
        }
        chars.push('#');
        chars.push('$');
        let len = chars.len();

        let mut max_center = 0;

        let mut res = vec![0; len];
        let mut center = 0;
        let mut boundary = 0;

        // 0 <= 2*center-boundary <= center <= i <= boundary
        // res[center] == boundary - center

        for index in 0..len {
            if index > boundary {
                center = index;
                boundary = index;
            }
            let index_ = 2 * center - index;
            match res[index_].cmp(&(boundary - index)) {
                std::cmp::Ordering::Less => res[index] = res[index_],
                std::cmp::Ordering::Greater => res[index] = boundary - index,
                std::cmp::Ordering::Equal => {
                    center = index;
                    res[center] = boundary - center;
                    let mut boundary_ = 2 * center - boundary;
                    while chars.get(boundary_ - 1) == chars.get(boundary + 1) {
                        // assert!(chars.get(boundary_ - 1).is_some() && chars.get(boundary + 1).is_some());
                        res[center] += 1;
                        boundary_ -= 1;
                        boundary += 1;
                    }
                    if res[center] > res[max_center] {
                        max_center = center;
                    }
                }
            }
        }

        chars[max_center - res[max_center]..=max_center + res[max_center]]
            .iter()
            .filter(|ch| ch != &&'#')
            .collect()
    }
}
