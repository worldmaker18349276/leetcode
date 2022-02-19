pub trait Problem10 {
    // Given an input string s and a pattern p, implement regular expression
    // matching with support for '.' and '*' where:
    //     '.' Matches any single character.​​​​
    //     '*' Matches zero or more of the preceding element.
    // The matching should cover the entire input string (not partial).
    fn is_match(s: String, p: String) -> bool;
}

struct Solution;

impl Problem10 for Solution {
    fn is_match(s: String, p: String) -> bool {
        fn is_match_iter(
            mut s: impl Iterator<Item = char> + Clone,
            mut p: impl Iterator<Item = char> + Clone,
        ) -> bool {
            loop {
                match p.next() {
                    Some('*') => match p.next().unwrap() {
                        '.' => {
                            while !is_match_iter(s.clone(), p.clone()) {
                                match s.next() {
                                    Some(_) => {}
                                    _ => return false,
                                }
                            }
                            return true;
                        }
                        ch => {
                            while !is_match_iter(s.clone(), p.clone()) {
                                match s.next() {
                                    Some(ch_) if ch_ == ch => {}
                                    _ => return false,
                                }
                            }
                            return true;
                        }
                    },
                    Some('.') => match s.next() {
                        Some(_) => {}
                        _ => return false,
                    },
                    Some(ch) => match s.next() {
                        Some(ch_) if ch_ == ch => {}
                        _ => return false,
                    },
                    None => return matches!(s.next(), None),
                }
            }
        }

        is_match_iter(s.chars().rev(), p.chars().rev())
    }
}

struct SolutionDP;

impl Problem10 for SolutionDP {
    fn is_match(s: String, p: String) -> bool {
        let p = p.chars().collect::<Vec<_>>();
        let plen = p.len();

        // dp[i] := p[..i] match s[..n]
        let mut dp = vec![false; p.len() + 1];

        // rules:
        //   is_match("", "")
        //   !is_match("", p + "c")
        //   !is_match("", p + ".")
        //   !is_match(s + "c", "")
        //   is_match(s, p)  =>  is_match(s + "c", p + "c")
        //   is_match(s, p)  =>  is_match(s + "c", p + ".")
        //   is_match(s, p)  =>  is_match(s, p + "c*")  // c* match nothing
        //   is_match(s, p)  =>  is_match(s, p + ".*")  // .* match nothing
        //   is_match(s, p + "c")  =>  is_match(s, p + "c*")  // c* match c
        //   is_match(s, p + ".")  =>  is_match(s, p + ".*")  // .* match .
        //   is_match(s, p + "c*")  =>  is_match(s + "c", p + "c*")  // c* match c+
        //   is_match(s, p + ".*")  =>  is_match(s + "c", p + ".*")  // .* match .+

        // match empty string
        dp[0] = true;
        for (i, a) in p.iter().enumerate() {
            dp[i + 1] = match a {
                '*' => dp[i - 1],
                _ => false,
            };
        }

        let mut dp_prev = vec![false; p.len() + 1];
        for ch in s.chars() {
            // match `ch` additionally
            std::mem::swap(&mut dp_prev, &mut dp);
            dp[0] = false;
            for i in 0..plen {
                dp[i + 1] = if p[i] == '*' {
                    if p[i - 1] == ch || p[i - 1] == '.' {
                        dp[i - 1] || dp[i] || dp_prev[i + 1]
                    } else {
                        dp[i - 1] || dp[i]
                    }
                } else if p[i] == ch || p[i] == '.' {
                    dp_prev[i]
                } else {
                    false
                };
            }
        }

        dp[plen]
    }
}
