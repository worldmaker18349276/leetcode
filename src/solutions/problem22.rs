pub trait Problem22 {
    // Given n pairs of parentheses, write a function to generate all
    // combinations of well-formed parentheses.
    fn generate_parenthesis(n: i32) -> Vec<String>;
}

struct Solution;

impl Problem22 for Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut res = Vec::new();

        let mut prev = vec![true; n];
        prev.append(&mut vec![false; n]);

        'outer: loop {
            res.push(prev.iter().map(|b| if *b { '(' } else { ')' }).collect());

            let mut depth = 0;
            for (i, is_open) in prev.iter().enumerate() {
                if *is_open {
                    depth += 1;
                } else {
                    depth -= 1;
                }
                if !is_open && depth > 0 {
                    let j = (i + depth - 1) / 2;
                    prev[i] = true;
                    prev[j..i].fill(false);
                    prev[..j].fill(true);
                    continue 'outer;
                }
            }
            break;
        }

        res
    }
}
