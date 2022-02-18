pub trait Problem4 {
    // Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
    // The overall run time complexity should be O(log (m+n)).
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

struct Solution;

impl Problem4 for Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        struct MergeIter<A, B>
        where
            A: Iterator,
            A::Item: Copy + Ord,
            B: Iterator<Item = A::Item>,
        {
            iter1: A,
            iter2: B,
            item1: Option<A::Item>,
            item2: Option<A::Item>,
        }

        impl<A, B> Iterator for MergeIter<A, B>
        where
            A: Iterator,
            A::Item: Copy + Ord,
            B: Iterator<Item = A::Item>,
        {
            type Item = A::Item;

            fn next(&mut self) -> Option<Self::Item> {
                match (self.item1, self.item2) {
                    (None, None) => None,
                    (None, item) => {
                        self.item2 = self.iter2.next();
                        item
                    }
                    (item, None) => {
                        self.item1 = self.iter1.next();
                        item
                    }
                    (item1, item2) if item1 <= item2 => {
                        self.item1 = self.iter1.next();
                        item1
                    }
                    (_, item2) => {
                        self.item2 = self.iter2.next();
                        item2
                    }
                }
            }
        }

        impl<A, B> MergeIter<A, B>
        where
            A: Iterator,
            A::Item: Copy + Ord,
            B: Iterator<Item = A::Item>,
        {
            fn new(mut iter1: A, mut iter2: B) -> MergeIter<A, B> {
                let item1 = iter1.next();
                let item2 = iter2.next();
                MergeIter {
                    iter1,
                    iter2,
                    item1,
                    item2,
                }
            }
        }

        let n1 = nums1.len();
        let n2 = nums2.len();
        let half = (n1 + n2 - 1) / 2;
        let is_even = (n1 + n2) % 2 == 0;

        let mut merged = MergeIter::new(nums1.iter(), nums2.iter());
        for _ in 0..half {
            merged.next();
        }
        let mut value = *merged.next().unwrap() as f64;
        if is_even {
            value = (value + *merged.next().unwrap() as f64) / 2.0;
        }
        value
    }
}
