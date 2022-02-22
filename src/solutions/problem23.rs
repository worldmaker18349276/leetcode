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
        fn update_and_return<T, U>(var: &mut T, f: impl FnOnce(T) -> (T, U)) -> U where T: Default {
            let mut old = T::default();
            std::mem::swap(var, &mut old);
            let (new, res) = f(old);
            *var = new;
            res
        }

        let mut lists = lists;
        if lists.iter().all(Option::is_none) {
            None
        } else {
            let list = lists
                .iter_mut()
                .max_by_key(|list| list.as_ref().map(|node| -node.val))
                .unwrap();

            let value = update_and_return(list, |node| {
                let ListNode { val, next } = *node.unwrap();
                (next, val)
            });

            Some(Box::new(ListNode {
                val: value,
                next: Self::merge_k_lists(lists),
            }))
        }
    }
}
