//! ## 合并两个有序链表
//!
//! 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
//!
//! 取值范围：
//! - 两个链表的节点数量是 `[0, 50]`
//! - -100 <= node.val <= 100
//! - l1 和 l2 均按照非递减顺序排列

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

impl TryFrom<Vec<i32>> for ListNode {
    type Error = &'static str;

    fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(
                "Empty value cannot convert to LinkedList, use None type to repr it direclty!",
            );
        }

        let mut head = Box::new(ListNode::new(value[0]));
        let mut cur = &mut head;

        for &val in &value[1..] {
            let new_node = Box::new(ListNode::new(val));
            cur.next = Some(new_node);
            cur = cur.next.as_mut().unwrap();
        }
        Ok(*head)
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let list1_ref = list1.as_ref().unwrap();
        let list2_ref = list2.as_ref().unwrap();
        let (mut head, mut cur1, mut cur2) = if list1_ref.val < list2_ref.val {
            (
                Box::new(ListNode::new(list1_ref.val)),
                &list1_ref.next,
                &list2,
            )
        } else {
            (
                Box::new(ListNode::new(list2_ref.val)),
                &list1,
                &list2_ref.next,
            )
        };
        let mut cur = &mut head;
        let concat = |x: &Option<Box<ListNode>>, n: &mut Box<ListNode>| {
            let mut cur_node = n;
            let mut cur = x.as_ref();
            while let Some(node) = cur {
                cur_node.next = Some(Box::new(ListNode::new(node.val)));
                cur_node = cur_node.next.as_mut().unwrap();
                cur = node.next.as_ref();
            }
        };
        loop {
            if cur1.is_none() {
                // 第一个链表遍历完了
                concat(cur2, cur);
                break;
            } else if cur2.is_none() {
                // 第二个链表遍历完了
                concat(cur1, cur);
                break;
            } else {
                let cur1_val = cur1.as_ref().unwrap().val;
                let cur2_val = cur2.as_ref().unwrap().val;
                let node_val = if cur1_val < cur2_val {
                    cur1 = &cur1.as_ref().unwrap().next;
                    cur1_val
                } else {
                    cur2 = &cur2.as_ref().unwrap().next;
                    cur2_val
                };
                cur.next = Some(Box::new(ListNode::new(node_val)));
                cur = cur.next.as_mut().unwrap();
            }
        }

        Some(head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        answer: Option<Box<ListNode>>,
    }

    impl TestCase {
        fn new(l1: Vec<i32>, l2: Vec<i32>, answer: Vec<i32>) -> TestCase {
            let converter = |v| {
                return ListNode::try_from(v).map_or_else(|_v| None, |v| Some(Box::new(v)));
            };
            TestCase {
                l1: converter(l1),
                l2: converter(l2),
                answer: converter(answer),
            }
        }
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
            TestCase::new(vec![], vec![0], vec![0]),
            TestCase::new(vec![], vec![], vec![]),
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::merge_two_lists(c.l1, c.l2);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
