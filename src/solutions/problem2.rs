// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub trait Problem2 {
    // You are given two non-empty linked lists representing two non-negative integers. The digits
    // are stored in reverse order, and each of their nodes contains a single digit. Add the two
    // numbers and return the sum as a linked list.
    // You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

struct Solution;

impl Problem2 for Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, rest @ Some(_)) | (rest @ Some(_), None) => rest,
            (Some(node1), Some(node2)) => {
                let mut val = node1.val + node2.val;
                let mut next = Self::add_two_numbers(node1.next, node2.next);
                if val >= 10 {
                    next = Self::add_two_numbers(next, Some(Box::new(ListNode::new(1))));
                    val -= 10;
                }
                Some(Box::new(ListNode { val, next }))
            }
        }
    }
}

struct SolutionLoop;

impl Problem2 for SolutionLoop {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut res: Vec<i32> = vec![];
        let mut carried = false;

        let mut next = loop {
            match (l1, l2, carried) {
                (Some(node1), Some(node2), _) => {
                    let mut val = node1.val + node2.val;
                    if carried {
                        val += 1;
                    }
                    if val >= 10 {
                        carried = true;
                        val -= 10;
                    } else {
                        carried = false;
                    }
                    res.push(val);
                    l1 = node1.next;
                    l2 = node2.next;
                }
                (None, list, false) | (list, None, false) => {
                    break list;
                }
                (None, None, true) => {
                    res.push(1);
                    break None;
                }
                (None, Some(node), true) | (Some(node), None, true) => {
                    let mut val = node.val + 1;
                    if val >= 10 {
                        carried = true;
                        val -= 10;
                    } else {
                        carried = false;
                    }
                    res.push(val);
                    l1 = None;
                    l2 = node.next;
                }
            }
        };

        for val in res.into_iter().rev() {
            next = Some(Box::new(ListNode {val, next}));
        }
        next
    }
}
