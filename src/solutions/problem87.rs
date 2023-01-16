use std::slice::SliceIndex;

pub trait Problem87 {
    fn is_scramble(s1: String, s2: String) -> bool;
}

struct Solution;

impl Problem87 for Solution {
    fn is_scramble(s1: String, s2: String) -> bool {
        fn go(s1: &[char], s2: &[char]) -> bool {
            let length = s1.len();
            let mut i0 = 0;
            'l: while i0 < length {
                let mut stat1: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
                let mut stat2: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
                for (i, (c1, c2)) in s1.iter().zip(s2.iter()).enumerate().skip(i0) {
                    *stat1.entry(*c1).or_default() += 1;
                    *stat2.entry(*c2).or_default() += 1;
                    if stat1 == stat2 && go_rev(&s1[i0..i], &s2[i0..i]) {
                        i0 = i;
                        continue 'l;
                    }
                }
                return false;
            }
            true
        }

        fn go_rev(s1: &[char], s2: &[char]) -> bool {
            let length = s1.len();
            let mut i0 = 0;
            'l: while i0 < length {
                let mut stat1: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
                let mut stat2: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
                for (i, (c1, c2)) in s1.iter().zip(s2.iter().rev()).enumerate().skip(i0) {
                    *stat1.entry(*c1).or_default() += 1;
                    *stat2.entry(*c2).or_default() += 1;
                    if stat1 == stat2 && go(&s1[i0..i], &s2[length-i..length-i0]) {
                        i0 = i;
                        continue 'l;
                    }
                }
                return false;
            }
            true
        }

        let s1: Vec<_> = s1.chars().collect();
        let s2: Vec<_> = s2.chars().collect();
        go(&s1, &s2)
    }
}
