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
        fn is_match_iter(mut s: impl Iterator<Item=char> + Clone, mut p: impl Iterator<Item=char> + Clone) -> bool {
            loop {
                match p.next() {
                    Some('*') => {
                        match p.next().unwrap() {
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
                        }
                    }
                    Some('.') => {
                        match s.next() {
                            Some(_) => {}
                            _ => return false,
                        }
                    }
                    Some(ch) => {
                        match s.next() {
                            Some(ch_) if ch_ == ch => {}
                            _ => return false,
                        }
                    }
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

        let mut dp = vec![false; p.len()+1];

        // match empty string
        dp[0] = true;
        for (i, a) in p.iter().enumerate() {
            match a {
                '*' => dp[i+1] = dp[i-1],
                _ => dp[i+1] = false,
            }
        }

        for ch in s.chars() {
            // match `ch` additionally
            let dp_prev = dp.clone();
            dp[0] = false;
            for (i, a) in p.iter().enumerate() {
                match a {
                    '*' => {
                        dp[i+1] = dp[i-1] || dp[i];
                        if ch == p[i-1] || '.' == p[i-1] {
                            dp[i+1] = dp[i+1] || dp_prev[i+1];
                        }
                    }
                    '.' => dp[i+1] = dp_prev[i],
                    a if a == &ch => dp[i+1] = dp_prev[i],
                    _ => dp[i+1] = false,
                }
            }
        }
        
        dp[plen]
    }
}