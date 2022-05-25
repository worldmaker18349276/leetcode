pub trait Problem88 {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32);
}

struct Solution;

impl Problem88 for Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = (m + n) as usize;
        for y in nums2.iter().rev() {
            let k = nums1[..i].iter().rev().take_while(|x| y < x).count();
            for l in 1..=k {
                nums1[j-l] = nums1[i-l];
            }
            nums1[j-k-1] = *y;
            i -= k;
            j -= k + 1;
        }
    }
}
