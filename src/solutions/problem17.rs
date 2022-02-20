pub trait Problem17 {
    // Given a string containing digits from 2-9 inclusive, return all possible
    // letter combinations that the number could represent. Return the answer in
    // any order. A mapping of digit to letters (just like on the telephone
    // buttons) is given below. Note that 1 does not map to any letters.
    // 2: abc
    // 3: def
    // 4: ghi
    // 5: jkl
    // 6: mno
    // 7: pqrs
    // 8: tuv
    // 9: wxyz
    fn letter_combinations(digits: String) -> Vec<String>;
}

struct Solution;

impl Problem17 for Solution {
    fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        static MAPPING: [&[char]; 8] = [
            &['a', 'b', 'c'],
            &['d', 'e', 'f'],
            &['g', 'h', 'i'],
            &['j', 'k', 'l'],
            &['m', 'n', 'o'],
            &['p', 'q', 'r', 's'],
            &['t', 'u', 'v'],
            &['w', 'x', 'y', 'z'],
        ];

        let mut letters: Vec<_> = digits
            .bytes()
            .map(|b| MAPPING[(b - b'2') as usize])
            .map(|chars| (chars, chars.iter()))
            .collect();

        let mut word: Vec<_> = letters
            .iter_mut()
            .map(|(_, iter)| *iter.next().unwrap())
            .collect();
        let mut res = vec![word.iter().collect()];

        'outer: loop {
            for (i, (chars, iter)) in letters.iter_mut().enumerate() {
                match iter.next() {
                    Some(ch) => {
                        word[i] = *ch;
                        res.push(word.iter().collect());
                        continue 'outer;
                    }
                    None => {
                        *iter = chars.iter();
                        let ch = iter.next().unwrap();
                        word[i] = *ch;
                    }
                }
            }
            break;
        }

        res
    }
}
