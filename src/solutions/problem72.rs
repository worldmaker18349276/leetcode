pub trait Problem72 {
    fn min_distance(word1: String, word2: String) -> i32;
}

struct Solution;

impl Problem72 for Solution {
    fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<_> = word1.chars().collect();
        let word2: Vec<_> = word2.chars().collect();

        let mut p: Vec<i32> = (0..word2.len() as i32 + 1).collect();
        for c1 in word1.iter() {
            let mut p_old_prev = p[0]; // p_old[i2]
            p[0] += 1;
            for (i2, c2) in word2.iter().enumerate() {
                let p_old = p[i2+1];

                if c1 == c2 {
                    p[i2+1] = p_old_prev;
                } else {
                    p[i2+1] = p_old_prev.min(p_old).min(p[i2]) + 1;
                }

                p_old_prev = p_old;
            }
        }

        p.pop().unwrap()
    }
}
