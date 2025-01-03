use crate::{ListNode, Solution};

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            let lst = if node1.val < node2.val {
                &mut list1
            } else {
                &mut list2
            };

            cur.next = lst.take();
            cur = cur.next.as_mut()?;
            *lst = cur.next.take();
        }

        cur.next = list1.or(list2);
        dummy.next
    }
}
