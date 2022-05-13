
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

pub trait Problem82 {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

struct Solution;

impl Problem82 for Solution {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn go(prev: i32, head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, bool) {
            match head {
                None => (None, false),
                Some(b) => {
                    let val = b.val;
                    let (next, res) = go(val, b.next);
                    if val == prev {
                        (next, true)
                    } else if res {
                        (next, false)
                    } else {
                        (Some(Box::new(ListNode {val, next})), false)
                    }
                }
            }
        }

        go(i32::MAX, head).0
    }
}

