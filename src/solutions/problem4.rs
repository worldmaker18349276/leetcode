pub trait Problem4 {
    // Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
    // The overall run time complexity should be O(log (m+n)).
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

struct SolutionIndex;

impl Problem4 for SolutionIndex {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let half = (n1 + n2 - 1) / 2;
        let is_even = (n1 + n2) % 2 == 0;

        let mut i = 0;
        let mut j = 0;

        for _ in 0..half {
            if j >= n2 || i < n1 && nums1[i] <= nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        if is_even {
            let value1 = if j >= n2 || i < n1 && nums1[i] <= nums2[j] {
                i += 1;
                nums1[i - 1]
            } else {
                j += 1;
                nums2[j - 1]
            };
            let value2 = if j >= n2 || i < n1 && nums1[i] <= nums2[j] {
                nums1[i]
            } else {
                nums2[j]
            };
            (value1 + value2) as f64 / 2.0
        } else if j >= n2 || i < n1 && nums1[i] <= nums2[j] {
            nums1[i] as f64
        } else {
            nums2[j] as f64
        }
    }
}
