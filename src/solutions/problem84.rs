pub trait Problem84 {
    fn largest_rectangle_area(heights: Vec<i32>) -> i32;
}

struct Solution;

impl Problem84 for Solution {
    fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        let mut stack = Vec::<(usize, usize)>::new();

        let mut right = vec![0; n];
        for (i, &h) in heights.iter().enumerate() {
            let h = h as usize;
            let (Ok(n) | Err(n)) = stack.binary_search(&(h + 1, 0));
            for (_, j) in stack.drain(n..) {
                right[j] = i - j - 1;
            }
            stack.push((h, i));
        }
        for (_, j) in stack.drain(..) {
            right[j] = n - j - 1;
        }

        let mut left = vec![0; n];
        for (i, &h) in heights.iter().enumerate().rev() {
            let h = h as usize;
            let (Ok(n) | Err(n)) = stack.binary_search(&(h + 1, 0));
            for (_, j) in stack.drain(n..) {
                left[j] = j - i - 1;
            }
            stack.push((h, i));
        }
        for (_, j) in stack.drain(..) {
            left[j] = j;
        }

        let width = left.into_iter().zip(right).map(|(l, r)| l + r + 1);
        heights.into_iter().zip(width).map(|(h, w)| h * w as i32).max().unwrap()
    }
}
