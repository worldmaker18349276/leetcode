// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub trait Problem24 {
    // Given a linked list, swap every two adjacent nodes and return its head.
    // You must solve the problem without modifying the values in the list's
    // nodes (i.e., only nodes themselves may be changed.)
    fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

struct Solution;

impl Problem24 for Solution {
    fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(node1) => {
                let val1 = node1.val;
                match node1.next {
                    None => Some(Box::new(ListNode {
                        val: val1,
                        next: None,
                    })),
                    Some(node2) => {
                        let val2 = node2.val;
                        let next = Self::swap_pairs(node2.next);
                        Some(Box::new(ListNode {
                            val: val2,
                            next: Some(Box::new(ListNode { val: val1, next })),
                        }))
                    }
                }
            }
        }
    }
}
