pub trait Problem30 {
    // You are given a string s and an array of strings words of the same
    // length. Return all starting indices of substring(s) in s that is a
    // concatenation of each word in words exactly once, in any order, and
    // without any intervening characters. You can return the answer in any
    // order.
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32>;
}

struct Solution;

impl Problem30 for Solution {
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
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
            fn trim(&mut self, which: &T) {
                let mut index = 0;
                while index < self.0.len() {
                    if &self.0[index].0 == which {
                        self.0.swap_remove(index);
                    } else {
                        index += 1;
                    }
                }
                for (_, node) in self.0.iter_mut() {
                    node.trim(which);
                }
            }
        }

        let s = s.as_bytes();
        let words_and_lens: Vec<_> = words
            .iter()
            .map(|s| s.as_bytes())
            .map(|b| (b, b.len()))
            .collect();
        let total_len: usize = words_and_lens.iter().map(|(_, len)| len).sum();
        let max_len: usize = *words_and_lens.iter().map(|(_, len)| len).max().unwrap();
        let mut reached: Vec<Tree<usize>> = (0..max_len).map(|_| Tree::new()).collect();
        let mut res = Vec::new();

        for pos in 0..s.len() {
            let tree = reached[pos % max_len].clone();
            reached[pos % max_len] = Tree::new();
            if tree.depth() == words.len() {
                res.push((pos - total_len) as i32);
            }
            for (index, (word, len)) in words_and_lens.iter().enumerate() {
                if pos + len <= s.len() && &&s[pos..pos + len] == word {
                    let mut tree = tree.clone();
                    tree.trim(&index);
                    reached[(pos + len) % max_len].0.push((index, tree));
                }
            }
        }
        if reached[s.len() % max_len].depth() == words.len() {
            res.push((s.len() - total_len) as i32);
        }

        res
    }
}

#[test]
fn t() {
    let s = String::from("ababaab");
    let words = vec!["ab","ba","ba"].into_iter().map(String::from).collect();
    let solution = Solution::find_substring(s, words);
    assert_eq!(solution, vec![1]);
}
