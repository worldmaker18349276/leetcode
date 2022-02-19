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
