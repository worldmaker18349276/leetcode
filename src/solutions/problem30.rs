pub trait Problem30 {
    // You are given a string s and an array of strings words of the same
    // length. Return all starting indices of substring(s) in s that is a
    // concatenation of each word in words exactly once, in any order, and
    // without any intervening characters. You can return the answer in any
    // order.
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32>;
}

struct SolutionOverpower;

impl Problem30 for SolutionOverpower {
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;
        use std::mem::replace;

        #[derive(Clone)]
        struct Tree<T>(Vec<(T, Tree<T>)>);

        impl<T> Tree<T> {
            fn new() -> Self {
                Tree(Vec::new())
            }

            fn depth(&self) -> usize {
                self.0.iter().map(|(_, n)| 1 + n.depth()).max().unwrap_or(0)
            }
        }

        impl<T: Eq> Tree<T> {
            fn trim(&mut self, which: &T, count: usize) {
                // debug_assert!(count > 0);
                if count == 1 {
                    let mut index = 0;
                    while index < self.0.len() {
                        if &self.0[index].0 == which {
                            self.0.swap_remove(index);
                        } else {
                            index += 1;
                        }
                    }
                    for (_, node) in self.0.iter_mut() {
                        node.trim(which, count);
                    }
                } else {
                    for (value, node) in self.0.iter_mut() {
                        let count = if value == which { count - 1 } else { count };
                        node.trim(which, count);
                    }
                }
            }
        }

        let s = s.as_bytes();
        let str_len = s.len();
        let mut dict = HashMap::new();
        for word in words.iter().map(String::as_bytes) {
            let entry = dict.entry(word).or_insert_with(|| (word.len(), 0));
            (*entry).1 += 1;
        }
        let dict: Vec<_> = dict
            .into_iter()
            .map(|(word, (len, count))| (word, len, count))
            .collect();

        let word_nums: usize = dict.iter().map(|(_, _, count)| count).sum();
        let total_len: usize = dict.iter().map(|(_, len, count)| len * count).sum();
        let max_len = *dict.iter().map(|(_, len, _)| len).max().unwrap();

        let mut reached: Vec<Tree<_>> = (0..max_len).map(|_| Tree::new()).collect();
        let mut res = Vec::new();

        for pos in 0..str_len {
            let tree = replace(&mut reached[pos % max_len], Tree::new());
            if tree.depth() == word_nums {
                res.push((pos - total_len) as i32);
            }
            for (index, &(word, len, count)) in dict.iter().enumerate() {
                if pos + len <= str_len && &s[pos..pos + len] == word {
                    let mut tree = tree.clone();
                    tree.trim(&index, count);
                    reached[(pos + len) % max_len].0.push((index, tree));
                }
            }
        }
        if reached[str_len % max_len].depth() == word_nums {
            res.push((str_len - total_len) as i32);
        }

        res
    }
}

struct Solution;

impl Problem30 for Solution {
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;

        let s = s.as_bytes();
        let str_len = s.len();
        let word_len = words[0].len();
        let word_nums = words.len();
        let total_len = word_len * word_nums;

        if total_len > str_len {
            return vec![];
        }

        let dict = {
            let mut dict = HashMap::new();
            let mut index = 0;
            for word in words.iter().map(String::as_bytes) {
                let entry = dict.entry(word).or_insert_with(|| {
                    index += 1;
                    (index, 0)
                });
                (*entry).1 += 1;
            }
            dict
        };

        let mut res = Vec::new();

        for offset in 0..word_len {
            let offset_end = offset + (str_len - offset) / word_len * word_len;
            let mut reached = vec![0; word_nums];
            for (pos, word) in s[offset..offset_end].chunks(word_len).enumerate() {
                if let Some(&(index, count)) = dict.get(word) {
                    reached[pos % word_nums] = index;
                    let finished = {
                        let mut finished = true;
                        let mut reached_count = 0;
                        for n in (1..word_nums+1).rev() {
                            let n = (pos + n) % word_nums;
                            if reached[n] == index {
                                reached_count += 1;
                            }
                            if reached_count > count {
                                reached[n] = 0;
                            }
                            if reached[n] == 0 {
                                finished = false;
                                break;
                            }
                        }
                        finished
                    };
                    if finished {
                        res.push((offset + (pos + 1) * word_len - total_len) as i32);
                    }
                } else {
                    reached[pos % word_nums] = 0;
                }
            }
        }

        res
    }
}
