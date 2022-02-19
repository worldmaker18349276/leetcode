pub trait Problem11 {
    // You are given an integer array height of length n. There are n vertical
    // lines drawn such that the two endpoints of the ith line are (i, 0) and
    // (i, height[i]). Find two lines that together with the x-axis form a
    // container, such that the container contains the most water. Return the
    // maximum amount of water a container can store. Notice that you may not
    // slant the container.
    fn max_area(height: Vec<i32>) -> i32;
}

struct Solution;
impl Problem11 for Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let mut forward = height.iter();
        let mut backward = height.iter().rev();
        let mut left = forward.next().unwrap();
        let mut right = backward.next().unwrap();
        let mut width = height.len() - 1;
        let mut area = 0;
        while width > 0 {
            let mut threshold = &0;
            if left <= right {
                while width > 0 && left <= right {
                    if left > threshold {
                        threshold = left;
                        area = area.max(left.min(right) * width as i32);
                    }
                    left = forward.next().unwrap();
                    width -= 1;
                }
            } else {
                while width > 0 && right <= left {
                    if right > threshold {
                        threshold = right;
                        area = area.max(right.min(left) * width as i32);
                    }
                    right = backward.next().unwrap();
                    width -= 1;
                }
            }
        }
        area
    }
}
