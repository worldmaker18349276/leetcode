pub trait Problem44 {
    fn is_match(s: String, p: String) -> bool;
}

struct Solution;

impl Problem44 for Solution {
    fn is_match(s: String, p: String) -> bool {
        let s: Vec<_> = s.chars().collect();

        let mut r = vec![false; s.len() + 1];
        r[0] = true;

        for c in p.chars() {
            match c {
                c @ 'a'..='z' => {
                    for j in (1..r.len()).rev() {
                        r[j] = r[j - 1] && s[j - 1] == c;
                    }
                    r[0] = false;
                }
                '?' => {
                    for j in (1..r.len()).rev() {
                        r[j] = r[j - 1];
                    }
                    r[0] = false;
                }
                '*' => {
                    if let Some((j, _)) = r.iter().enumerate().find(|(_, a)| **a) {
                        r[j..].fill(true);
                    }
                }
                _ => {
                    panic!();
                }
            }
        }

        r.pop().unwrap()
    }
}
