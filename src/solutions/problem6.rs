pub trait Problem6 {
    // The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows
    // like this: (you may want to display this pattern in a fixed font for better legibility)
    // P   A   H   N
    // A P L S I I G
    // Y   I   R
    // And then read line by line: "PAHNAPLSIIGYIR"
    // Write the code that will take a string and make this conversion given a number of rows:
    // string convert(string s, int numRows);
    fn convert(s: String, num_rows: i32) -> String;
}

struct Solution;

impl Problem6 for Solution {
    fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        let chars = s.chars().collect::<Vec<_>>();
        let len = chars.len();
        let mut res = Vec::new();

        let mut i = 0;
        while i < len {
            res.push(chars[i]);
            i += num_rows * 2 - 2;
        }

        for i0 in 1..num_rows - 1 {
            i = i0;
            while i < len {
                res.push(chars[i]);
                i += num_rows * 2 - 2 - 2 * i0;
                if i < len {
                    res.push(chars[i]);
                }
                i += 2 * i0;
            }
        }

        i = num_rows - 1;
        while i < len {
            res.push(chars[i]);
            i += num_rows * 2 - 2;
        }

        res.into_iter().collect()
    }
}

struct SolutionIter;

impl Problem6 for SolutionIter {
    fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        let mut res = Vec::new();
        let s: Vec<_> = s.chars().collect();

        for c in s.iter().step_by(num_rows * 2 - 2) {
            res.push(c);
        }

        for i0 in 1..num_rows - 1 {
            let iter1 = s.iter().skip(i0).step_by(num_rows * 2 - 2);
            let mut iter2 = s.iter().skip(num_rows * 2 - 2 - i0).step_by(num_rows * 2 - 2);
            for c in iter1 {
                res.push(c);
                if let Some(c) = iter2.next() {
                    res.push(c);
                }
            }
        }

        for c in s.iter().skip(num_rows - 1).step_by(num_rows * 2 - 2) {
            res.push(c);
        }

        res.into_iter().collect()
    }
}
