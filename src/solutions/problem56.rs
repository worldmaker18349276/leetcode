pub trait Problem56 {
    fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution;

impl Problem56 for Solution {
    fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn length(mut head: &Option<Box<ListNode>>) -> usize {
            let mut len = 0;
            loop {
                match head {
                    &Some(ref node) => {
                        len += 1;
                        head = &node.next;
                    }
                    None => return len,
                }
            }
        }

        let mut head = head;
        let len = length(&head);
        if len == 0 {
            return head;
        }
        let n = len - k as usize % len;
        if n == len {
            return head;
        }

        let mut curr = &mut head;
        for _ in 0..n {
            curr = &mut curr.as_mut().unwrap().next;
        }
        let mut new_head = std::mem::take(curr);

        let mut curr = &mut new_head;
        for _ in 0..len-n {
            curr = &mut curr.as_mut().unwrap().next;
        }
        *curr = head;

        new_head
    }
}
