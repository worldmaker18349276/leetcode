pub trait Problem68 {
    fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String>;
}

struct Solution;

impl Problem68 for Solution {
    fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut lines: Vec<(Vec<String>, usize)> = Vec::new();
        let mut line: Vec<String> = Vec::new();
        let mut pos = 0;
        let mut width = 0;

        for word in words.into_iter() {
            let len = word.len();
            pos += len;
            if pos > max_width {
                lines.push((line, width));
                line = Vec::new();
                pos = len;
                width = 0;
            }
            line.push(word);
            pos += 1;
            width += len;
        }
        if !line.is_empty() {
            lines.push((line, width));
        }

        let (mut last_line, _) = lines.pop().unwrap();
        let mut res = Vec::new();

        for (mut line, width) in lines {
            let mut line_res = Vec::new();
            let num = line.len() - 1;
            let spaces = max_width - width;
            let last_word = line.pop().unwrap();
            if num == 0 {
                let space: String = vec![' '; spaces].into_iter().collect();
                res.push(format!("{}{}", last_word, space));
                continue;
            }
            let w = spaces % num;
            let sp = spaces / num;
            for (i, word) in line.into_iter().enumerate() {
                line_res.push(word);
                if i < w {
                    line_res.push(vec![' '; sp + 1].into_iter().collect());
                } else {
                    line_res.push(vec![' '; sp].into_iter().collect());
                }
            }
            line_res.push(last_word);
            res.push(line_res.concat());
        }

        {
            let mut line_res = Vec::new();
            let mut total_width = 0;
            let last_word = last_line.pop().unwrap();
            for word in last_line {
                total_width += word.len() + 1;
                line_res.push(word);
                line_res.push(String::from(" "));
            }
            total_width += last_word.len();
            line_res.push(last_word);
            line_res.push(vec![' '; max_width - total_width].into_iter().collect());
            res.push(line_res.concat());
        }
        
        res
    }
}
