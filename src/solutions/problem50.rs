pub trait Problem50 {
    fn my_pow(x: f64, n: i32) -> f64;
}

struct Solution;

impl Problem50 for Solution {
    fn my_pow(x: f64, n: i32) -> f64 {
        if n == -2147483648 && x > 1.0 {
            return 0.0;
        }

        if x == 0.0 || x == 1.0 {
            return x;
        }
        if x == -1.0 {
            if n % 2 == 0 {
                return 1.0;
            } else {
                return -1.0;
            }
        }

        const N: i32 = 2;
        match n.cmp(&0) {
            std::cmp::Ordering::Less => Self::my_pow(1.0 / x, -n),
            std::cmp::Ordering::Equal => 1.0,
            std::cmp::Ordering::Greater => {
                let y = Self::my_pow(x, n / N);
                (0..N).fold(1.0, |acc, _| acc * y) * (0..n % N).fold(1.0, |acc, _| acc * x)
            }
        }
    }
}

struct SolutionLoop;

impl Problem50 for SolutionLoop {
    fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n;
        let mut res = 1.0;

        if n < 0 {
            x = 1.0 / x;
            if n == i32::MIN {
                res *= x;
                n = i32::MAX;
            } else {
                n = -n;
            }
        }

        while n != 0 {
            if n % 2 == 1 {
                res *= x;
            }
            x = x * x;
            n = n / 2;
        }

        return res;
    }
}
