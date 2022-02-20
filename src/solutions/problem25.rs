// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub trait Problem25 {
    // Given the head of a linked list, reverse the nodes of the list k at a
    // time, and return the modified list. k is a positive integer and is less
    // than or equal to the length of the linked list. If the number of nodes is
    // not a multiple of k then left-out nodes, in the end, should remain as it
    // is. You may not alter the values in the list's nodes, only nodes
    // themselves may be changed.
    fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

struct Solution;

impl Problem25 for Solution {
    fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn eat(head: Option<Box<ListNode>>, k: i32) -> (Option<Vec<i32>>, Option<Box<ListNode>>) {
            if k == 0 {
                (Some(vec![]), head)
            } else {
                match head {
                    None => (None, head),
                    Some(node) => {
                        let val = node.val;
                        match eat(node.next, k-1) {
                            (None, next) => (None, Some(Box::new(ListNode { val, next }))),
                            (Some(mut food), next) => {
                                food.push(val);
                                (Some(food), next)
                            }
                        }
                    }
                }
            }
        }

        fn feed(mut head: Vec<i32>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut next = tail;
            while let Some(val) = head.pop() {
                next = Some(Box::new(ListNode { val, next }))
            }
            next
        }

        match eat(head, k) {
            (None, tail) => tail,
            (Some(food), tail) => {
                feed(food, Self::reverse_k_group(tail, k))
            }
        }
    }
}
