pub trait Problem56 {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem56 for Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        for interval in intervals.into_iter() {
            let mut a = interval[0];
            let mut b = interval[1];
            let i = match res.binary_search(&a) {
                Ok(i) => {
                    if i % 2 == 0 {
                        i
                    } else {
                        a = res[i-1];
                        i - 1
                    }
                }
                Err(i) => {
                    if i % 2 == 0 {
                        i
                    } else {
                        a = res[i-1];
                        i - 1
                    }
                }
            };
            let j = match res.binary_search(&b) {
                Ok(j) => {
                    if j % 2 == 0 {
                        b = res[j+1];
                        j + 2
                    } else {
                        j + 1
                    }
                }
                Err(j) => {
                    if j % 2 == 0 {
                        j
                    } else {
                        b = res[j];
                        j + 1
                    }
                }
            };
            res.drain(i..j);
            res.insert(i, a);
            res.insert(i+1, b);
        }
        res.chunks(2).map(|v| v.to_owned()).collect()
    }
}
