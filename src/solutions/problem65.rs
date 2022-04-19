pub trait Problem65 {
    fn is_number(s: String) -> bool;
}

struct Solution;

impl Problem65 for Solution {
    fn is_number(s: String) -> bool {
        fn parse_sign(i: usize, chars: &[char]) -> Result<usize, ()> {
            if i >= chars.len() {
                return Err(());
            }
            match chars[i] {
                '+' | '-' => Ok(i+1),
                _ => Err(()),
            }
        }

        fn parse_dot(i: usize, chars: &[char]) -> Result<usize, ()> {
            if i >= chars.len() {
                return Err(());
            }
            match chars[i] {
                '.' => Ok(i+1),
                _ => Err(()),
            }
        }

        fn parse_e(i: usize, chars: &[char]) -> Result<usize, ()> {
            if i >= chars.len() {
                return Err(());
            }
            match chars[i] {
                'e' | 'E' => Ok(i+1),
                _ => Err(()),
            }
        }

        fn parse_digits(i: usize, chars: &[char]) -> Result<usize, ()> {
            if i >= chars.len() {
                return Err(());
            }
            match chars[i] {
                '0'..='9' => parse_digits(i+1, chars).or(Ok(i+1)),
                _ => Err(()),
            }
        }

        fn parse_int(i: usize, chars: &[char]) -> Result<usize, ()> {
            let i = parse_sign(i, chars).unwrap_or(i);
            parse_digits(i, chars)
        }

        fn parse_dec(i: usize, chars: &[char]) -> Result<usize, ()> {
            let i = parse_sign(i, chars).unwrap_or(i);

            match parse_digits(i, chars) {
                Ok(i) => {
                    let i = parse_dot(i, chars)?;
                    parse_digits(i, chars).or(Ok(i))
                }
                Err(()) => {
                    let i = parse_dot(i, chars)?;
                    parse_digits(i, chars)
                }
            }
        }

        fn parse_num(i: usize, chars: &[char]) -> Result<usize, ()> {
            let i = parse_dec(i, chars).or_else(|_| parse_int(i, chars))?;

            match parse_e(i, chars) {
                Ok(i) => parse_int(i, chars),
                Err(_) => Ok(i),
            }
        }

        let chars: Vec<_> = s.chars().collect();
        parse_num(0, &chars) == Ok(chars.len())
    }
}
