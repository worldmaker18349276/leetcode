// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub trait Problem23 {
    // You are given an array of k linked-lists lists, each linked-list is
    // sorted in ascending order. Merge all the linked-lists into one sorted
    // linked-list and return it.
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>;
}

struct Solution;

impl Problem23 for Solution {
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn vec_update<T, F>(list: &mut Vec<T>, index: usize, f: F)
        where
            F: FnOnce(T) -> T,
        {
            let element = list.remove(index);
            let element = f(element);
            list.insert(index, element);
        }

        let mut lists = lists;
        if lists.iter().all(Option::is_none) {
            None
        } else {
            let index = lists
                .iter()
                .enumerate()
                .max_by_key(|(_, list)| list.as_ref().map(|node| -node.val))
                .unwrap()
                .0;

            let mut value = 0;
            vec_update(&mut lists, index, |node| {
                let ListNode { val, next } = *node.unwrap();
                value = val;
                next
            });

            Some(Box::new(ListNode {
                val: value,
                next: Self::merge_k_lists(lists),
            }))
        }
    }
}
