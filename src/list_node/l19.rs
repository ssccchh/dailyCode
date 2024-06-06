/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 */

use crate::Solution;

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn _remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_p = &mut dummy;
        let mut fast_p = &slow_p.clone();

        // 快指针先移动到第n个节点
        for _ in 0..=n {
            if let Some(fast_node) = fast_p {
                fast_p = &fast_node.next;
            } else {
                return None;
            }
        }

        // 快慢指针同时移动
        while fast_p.is_some() {
            fast_p = &fast_p.as_ref().unwrap().next;
            slow_p = &mut slow_p.as_mut().unwrap().next;
        }

        let remove_p = &mut slow_p.as_mut().unwrap().next;
        slow_p.as_mut().unwrap().next = remove_p.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }
}
// @lc code=end
