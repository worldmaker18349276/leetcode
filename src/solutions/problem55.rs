pub trait Problem55 {
    fn get_permutation(n: i32, k: i32) -> String;
}

struct Solution;

impl Problem55 for Solution {
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
