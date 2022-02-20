// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub trait Problem19 {
    // Given the head of a linked list, remove the nth node from the end of the list and return its head.
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>>;
}

struct Solution;

impl Problem19 for Solution {
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn go(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
            match head {
                None => (head, n-1),
                Some(node) => {
                    let val = node.val;
                    let (next, m) = go(node.next, n);
                    if m == 0 {
                        (next, m-1)
                    } else {
                        (Some(Box::new(ListNode { next, val })), m-1)
                    }
                }
            }
        }

        go(head, n).0
    }
}
