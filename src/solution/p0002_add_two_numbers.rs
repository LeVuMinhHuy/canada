/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 * <strong class="example">Example 2:
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 * <strong class="example">Example 3:
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        /*
        why we can't use l1 and l2 directly?
        ====================================
        because we need to traverse the list l1 and l2
        that means we need to move the pointer of l1 and l2
        but if l1 and l2 is immutable like the input
        we cannot re assign the pointer l1, e.g l1 = l1.next
        so we shadow the l1 and l2 with mut l1 and mut l2
        in order to traverse the list
        */

        let mut l1 = l1;
        let mut l2 = l2;

        let mut head = Some(Box::new(ListNode::new(0)));

        /*
        why we need to create a dummy node?
        ==================================
        because we need to return the head of the list
        but we don't know the head of the list before we traverse the list
        so we create a dummy node, and return the next of the dummy node
        the next of the dummy node is the head of the list
        the dummy node is just a placeholder
        */

        // create a pointer as head to traverse the list
        let mut pointer = &mut head;

        let mut carry = 0;

        // we don't care about the length of l1 and l2
        // we will traverse both l1 and l2 until both l1 and l2 is None
        // also if they are both None, we still need to check if there is a carry
        // if there is a carry, we need to add a new node to the list
        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;
            sum %= 10;

            let new_node = ListNode::new(sum);

            // head.next should point to the new_node
            // but we want to keep the dummy head at the beginning
            // if we do the action on the head, e.g move the head: head = head.next
            // we will lose the beginning of the list
            // and we already have a pointer to traverse the list, that is pointer
            // then we use as_mut(), turn pointer into head and do the head actions
            // so that we can keep the beginning of the list
            pointer.as_mut().unwrap().next = Some(Box::new(new_node));
            pointer = &mut pointer.as_mut().unwrap().next;
        }

        // simple return head.next
        // but head is an Option
        // so we need to unwrap the head
        head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        );
        assert_eq!(
            Solution::add_two_numbers(
                to_list(vec![9, 9, 9, 9, 9, 9, 9]),
                to_list(vec![9, 9, 9, 9])
            ),
            to_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
