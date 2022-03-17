pub trait Problem47 {
    fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

struct Solution;

impl Problem47 for Solution {
    fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn go(res: &mut Vec<Vec<i32>>, perm: Vec<i32>, nums: &[i32], remains: Vec<usize>, length: usize) {
            if length == 0 {
                res.push(perm);
                return;
            }
            for (i, v) in remains.iter().enumerate() {
                if v > &0 {
                    let mut remains = remains.clone();
                    remains[i] -= 1;
                    let mut perm = perm.clone();
                    perm.push(nums[i]);
                    go(res, perm, nums, remains, length - 1);
                }
            }
        }

        let mut nums_unique = Vec::new();
        let mut remains = Vec::new();
        for n in nums.iter() {
            match nums_unique.binary_search(n) {
                Ok(i) => {
                    remains[i] += 1;
                }
                Err(i) => {
                    nums_unique.insert(i, *n);
                    remains.insert(i, 1);
                }
            }
        }

        let mut res = Vec::new();
        go(&mut res, vec![], &nums_unique, remains, nums.len());
        res
    }
}
