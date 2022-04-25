pub trait Problem67 {
    fn add_binary(a: String, b: String) -> String;
}

struct Solution;

impl Problem67 for Solution {
    fn add_binary(a: String, b: String) -> String {
        let a_len = a.len();
        let b_len = b.len();
        let a_tail = if a_len > b_len { 0 } else { b_len - a_len };
        let b_tail = if a_len > b_len { a_len - b_len } else { 0 };
        let a = a
            .chars()
            .rev()
            .map(|c| c == '1')
            .chain(vec![false; a_tail].into_iter());
        let b = b
            .chars()
            .rev()
            .map(|c| c == '1')
            .chain(vec![false; b_tail].into_iter());

        let mut c = false;
        let mut res: Vec<_> = a
            .zip(b)
            .map(|(a, b)| {
                let r = a ^ b ^ c;
                c = a && b || b && c || a && c;
                r
            })
            .collect();
        if c {
            res.push(true);
        }
        res.into_iter()
            .rev()
            .map(|r| if r { '1' } else { '0' })
            .collect()
    }
}
