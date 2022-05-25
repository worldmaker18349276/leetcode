pub trait Problem86 {
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>>;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

struct Solution;

impl Problem86 for Solution {
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut left = None;
        let mut middle = &mut left;
        let mut right = &mut head;
        while let Some(boxed) = right.as_mut() {
            if boxed.val < x {
                *middle = right.take();
                middle = &mut middle.as_mut().unwrap().next;
                *right = middle.take();
            } else {
                right = &mut right.as_mut().unwrap().next;
            }
        }
        *middle = head;
        left
    }
}
