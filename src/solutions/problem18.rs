pub trait Problem18 {
    // Given an array nums of n integers, return an array of all the unique
    // quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
    //     0 <= a, b, c, d < n
    //     a, b, c, and d are distinct.
    //     nums[a] + nums[b] + nums[c] + nums[d] == target
    // You may return the answer in any order.
    fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem18 for Solution {
    fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        struct UniqueIterator<I>
        where
            I: Iterator,
            I::Item: Copy + PartialEq,
        {
            origin: I,
            prev: Option<I::Item>,
            
        }

        impl<I> Iterator for UniqueIterator<I>
        where
            I: Iterator,
            I::Item: Copy + PartialEq,
        {
            type Item = I::Item;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    let next = self.origin.next();
                    if next.is_none() || next != self.prev {
                        self.prev = next;
                        return next;
                    }
                }
            }
        }

        impl<I> UniqueIterator<I>
        where
            I: Iterator,
            I::Item: Copy + PartialEq,
        {
            fn new(origin: I) -> Self {
                UniqueIterator { origin, prev: None }
            }
        }

        use std::cmp::Ordering;

        let mut res = Vec::new();

        let mut nums = nums;
        nums.sort_unstable();
        let mut iter1 = UniqueIterator::new(nums.iter());
        while let Some(x) = iter1.next() {
            let mut iter2 = UniqueIterator::new(iter1.origin.clone());
            while let Some(y) = iter2.next() {
                let iter3 = iter2.origin.clone().enumerate();
                let mut forward = UniqueIterator::new(iter3.clone());
                let mut backward = UniqueIterator::new(iter3.rev());
                let mut left = forward.next();
                let mut right = backward.next();
                while let (Some((k, z)), Some((l, w))) = (left, right) {
                    if k >= l {
                        break;
                    }
                    match (x + y + z + w).cmp(&target) {
                        Ordering::Less => left = forward.next(),
                        Ordering::Greater => right = backward.next(),
                        Ordering::Equal => {
                            let quadruplet = vec![*x, *y, *z, *w];
                            if !res.contains(&quadruplet) {
                                res.push(quadruplet);
                            }
                            left = forward.next();
                            right = backward.next();
                        }
                    }
                }
            }
        }

        res
    }
}
