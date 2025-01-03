use crate::{ListNode, Solution};

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // 虚拟头节点
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;

        for _ in 0..left - 1 {
            if let Some(prev_node) = prev {
                prev = &mut prev_node.next;
            }
        }

        // 从链表中取出第一个要反转的节点，注意要用take获取到节点的所有权
        let mut start = prev.as_mut()?.next.take();

        // 当前处理的节点，默认为起始节点的下一个节点
        let mut current = start.as_mut()?.next.take();

        // 开始反转
        for _ in left..right {
            if let Some(mut node) = current {
                // 更新current
                current = node.next;
                // 当前节点指向start
                node.next = start;
                // 更新start为当前节点，继续下一次的反转
                start = Some(node);
            }
        }

        // 重新连接子链表，注意在rust中，由于所有权的问题，我们在做反转的时候影响了原链表，需要遍历子链表，找到子链表的最后一个节点，做链接
        prev.as_mut()?.next = start;
        let mut tail = prev.as_mut()?.next.as_mut();
        while let Some(node) = tail {
            if node.next.is_none() {
                node.next = current;
                break;
            }
            tail = node.next.as_mut();
        }

        dummy?.next
    }
}
