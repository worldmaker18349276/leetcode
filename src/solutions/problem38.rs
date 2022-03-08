pub trait Problem38 {
    fn count_and_say(n: i32) -> String;
}

struct Solution;

impl Problem38 for Solution {
    fn count_and_say(n: i32) -> String {
        let mut say = vec![1];
        for _ in 0..n - 1 {
            let mut res = vec![];
            let mut n0 = say[0];
            let mut count = 0;
            for n in say.into_iter() {
                if n == n0 {
                    count += 1;
                } else {
                    let i = res.len();
                    res.insert(i, count % 10);
                    while count >= 10 {
                        count /= 10;
                        res.insert(i, count % 10);
                    }
                    res.push(n0);
                    n0 = n;
                    count = 1;
                }
            }
            let i = res.len();
            res.insert(i, count % 10);
            while count >= 10 {
                count /= 10;
                res.insert(i, count % 10);
            }
            res.push(n0);
            say = res;
        }
        say.into_iter().map(|d| d.to_string()).collect::<String>()
    }
}
