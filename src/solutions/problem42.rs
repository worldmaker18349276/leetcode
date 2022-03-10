pub trait Problem42 {
    fn trap(height: Vec<i32>) -> i32;
}

struct Solution;

impl Problem42 for Solution {
    fn trap(height: Vec<i32>) -> i32 {
        let mut left: Vec<(usize, i32)> = Vec::new();
        let mut volume = 0;
        for (i, h) in height.iter().enumerate() {
            if let Some((n, _)) = left.iter().enumerate().find(|(_, (_, v))| v < h) {
                if n > 0 {
                    volume += (i - 1 - left[n - 1].0) as i32 * (h - left[n].1);
                }
                volume += left[n..]
                    .windows(2)
                    .map(|w| (i - 1 - w[0].0) as i32 * (w[0].1 - w[1].1))
                    .sum::<i32>();
                left.drain(n..);
            }
            left.push((i, *h));
        }
        volume
    }
}
