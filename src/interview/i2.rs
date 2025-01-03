use crate::{ListNode, Solution};

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Self::add_two(l1, l2, 0)
        let mut dummy = ListNode::new(0); // 哨兵节点
        let mut cur = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            if let Some(node) = l1 {
                carry += node.val; // 节点值和进位相加
                l1 = node.next;
            }
            if let Some(node) = l2 {
                carry += node.val;
                l2 = node.next;
            }
            cur.next = Some(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            cur = cur.next.as_mut()?;
        }
        dummy.next
    }

    // pub fn add_two(
    //     l1: Option<Box<ListNode>>,
    //     l2: Option<Box<ListNode>>,
    //     carry: i32,
    // ) -> Option<Box<ListNode>> {
    //     match (l1, l2) {
    //         // 遍历到边界
    //         (None, None) => {
    //             if carry == 0 {
    //                 return None;
    //             }
    //             Some(Box::new(ListNode::new(carry)))
    //         }
    //         // l1到头, 交换l1和l2，保证l1非空
    //         (None, Some(node2)) => Self::add_two(Some(node2), None, carry),
    //         // l2到头
    //         (Some(mut node1), None) => {
    //             let sum = node1.val + carry;
    //             node1.val = sum % 10;
    //             node1.next = Self::add_two(node1.next.take(), None, sum / 10);
    //             Some(node1)
    //         }
    //         // 正常递归
    //         (Some(mut node1), Some(mut node2)) => {
    //             let sum = node1.val + node2.val + carry;
    //             node1.val = sum % 10;
    //             node1.next = Self::add_two(node1.next.take(), node2.next.take(), sum / 10);
    //             Some(node1)
    //         }
    //     }
    // }
}
