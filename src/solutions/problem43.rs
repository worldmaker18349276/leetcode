pub trait Problem43 {
    fn multiply(num1: String, num2: String) -> String;
}

struct Solution;

impl Problem43 for Solution {
    fn multiply(num1: String, num2: String) -> String {
        let num1: Vec<_> = num1.chars().rev().map(|c| c as u8 - b'0').collect();
        let num2: Vec<_> = num2.chars().rev().map(|c| c as u8 - b'0').collect();
        let mut res = Vec::new();

        fn add(res: &mut Vec<u8>, i: usize, n: u8) {
            if n == 0 {
                return;
            }
            while res.len() <= i {
                res.push(0);
            }
            let v = res[i] + n;
            res[i] = v % 10;
            add(res, i + 1, v / 10);
        }

        for (i, n) in num1.iter().enumerate() {
            for (j, m) in num2.iter().enumerate() {
                add(&mut res, i + j, n * m);
            }
        }
        if res.is_empty() {
            String::from("0")
        } else {
            res.into_iter().rev().map(|n| (n + b'0') as char).collect()
        }
    }
}
