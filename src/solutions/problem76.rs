pub trait Problem76 {
    fn min_window(s: String, t: String) -> String;
}

struct Solution;

impl Problem76 for Solution {
    fn min_window(s: String, t: String) -> String {
        let s: Vec<char> = s.chars().collect();

        let mut u = std::collections::HashMap::new();
        for c in t.chars() {
            let n = u.entry(c).or_insert((0, 0));
            n.0 += 1;
        }

        let keys: Vec<_> = u.keys().copied().collect();

        let mut right_iter = s.iter().enumerate().filter(|(_, c)| keys.contains(c));
        let mut left = match right_iter.next() {
            None => return String::from(""),
            Some((i, c)) => {
                u.get_mut(c).unwrap().1 += 1;
                (i, c)
            }
        };
        let mut left_iter = right_iter.clone();
        let mut min = if u.values().all(|n| n.1 >= n.0) {
            (left.0, left.0 + 1)
        } else {
            (left.0, left.0)
        };

        for right in right_iter {
            let n = u.get_mut(right.1).unwrap();
            n.1 += 1;
            if u.values().all(|n| n.1 >= n.0) {
                loop {
                    let n = u.get_mut(left.1).unwrap();
                    if n.1 == n.0 {
                        break;
                    }
                    n.1 -= 1;
                    left = left_iter.next().unwrap();
                }
                if right.0 + 1 - left.0 < min.1 - min.0 || min.0 == min.1 {
                    min = (left.0, right.0 + 1);
                }
            }
        }

        s[min.0..min.1].iter().collect()
    }
}
