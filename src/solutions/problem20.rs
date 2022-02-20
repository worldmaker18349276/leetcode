pub trait Problem20 {
    // Given a string s containing just the characters '(', ')', '{', '}', '[' and
    // ']', determine if the input string is valid.
    // An input string is valid if:
    //     Open brackets must be closed by the same type of brackets.
    //     Open brackets must be closed in the correct order.
    fn is_valid(s: String) -> bool;
}

struct Solution;

impl Problem20 for Solution {
    fn is_valid(s: String) -> bool {
        enum Bracket {
            Round,
            Square,
            Curvy,
        }

        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(Bracket::Round),
                '[' => stack.push(Bracket::Square),
                '{' => stack.push(Bracket::Curvy),
                ')' => {
                    if !matches!(stack.pop(), Some(Bracket::Round)) {
                        return false;
                    }
                }
                ']' => {
                    if !matches!(stack.pop(), Some(Bracket::Square)) {
                        return false;
                    }
                }
                '}' => {
                    if !matches!(stack.pop(), Some(Bracket::Curvy)) {
                        return false;
                    }
                }
                _ => panic!(),
            }
        }
        stack.is_empty()
    }
}