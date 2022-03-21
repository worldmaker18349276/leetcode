pub trait Problem49 {
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>;
}

struct Solution;

impl Problem49 for Solution {
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut keys: Vec<Vec<char>> = Vec::new();
        for str in strs.into_iter() {
            let mut key = str.chars().collect::<Vec<_>>();
            key.sort_unstable();
            match keys.binary_search(&key) {
                Ok(i) => {
                    res[i].push(str);
                }
                Err(i) => {
                    keys.insert(i, key);
                    res.insert(i, vec![str]);
                }
            }
        }
        res
    }
}
