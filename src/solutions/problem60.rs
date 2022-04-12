pub trait Problem60 {
    fn get_permutation(n: i32, k: i32) -> String;
}

struct Solution;

impl Problem60 for Solution {
    fn get_permutation(n: i32, k: i32) -> String {
        static CHARS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let n = n as usize;
        let mut perm: Vec<_> = (0..n).into_iter().collect();
        for _ in 0..k-1 {
            for i in (0..n).rev() {
                let val = &perm[i];
                match perm[i+1..].binary_search_by(|n: &usize| val.cmp(n)) {
                    Ok(_) => panic!(),
                    Err(0) => continue,
                    Err(j) => {
                        perm.swap(i, i+j);
                        perm[i+1..].reverse();
                        break;
                    }
                }
            }
        }
        perm.into_iter().map(|n| CHARS[n]).collect()
    }
}

struct SolutionCalc;

impl Problem60 for SolutionCalc {
    fn get_permutation(n: i32, k: i32) -> String {
        static CHARS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let n = n as usize;
        let mut k = (k - 1) as usize;
        let mut elems: Vec<_> = CHARS[..n].iter().collect();
        let mut res = Vec::new();

        let mut acc = 1;
        let factorials: Vec<_> = (1..=n).map(|n| { acc *= n; acc }).collect();
        for m in factorials.into_iter().rev().skip(1) {
            res.push(elems.remove(k / m));
            k %= m;
        }
        res.push(elems.pop().unwrap());
        res.into_iter().collect()
    }
}
