
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

pub trait Problem83 {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

struct Solution;

impl Problem83 for Solution {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn go(prev: i32, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head {
                None => None,
                Some(b) => {
                    let val = b.val;
                    let next = go(val, b.next);
                    if val == prev {
                        next
                    } else {
                        Some(Box::new(ListNode {val, next}))
                    }
                }
            }
        }

        go(i32::MAX, head)
    }
}

struct SolutionLoop;

impl Problem83 for SolutionLoop {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut curr = &mut head;
        let mut val = i32::MAX;

        while let Some(node) = curr.as_mut() {
            if val == node.val {
                *curr = std::mem::take(&mut node.next);
            } else {
                val = node.val;
                curr = &mut node.next;
            }
        }

        head
    }
}

