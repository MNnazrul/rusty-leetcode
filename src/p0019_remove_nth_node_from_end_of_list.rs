#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut cur = &head;
        let mut tot = 0;
        while let Some(node) = cur {
            tot += 1;
            cur = &node.next;
        }
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        for _ in 0..(tot - n) {
            cur = cur.next.as_mut().unwrap();
        }
        let next = cur.next.as_mut().unwrap().next.take();
        cur.next = next;

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in v.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn test_example_1() {
        // [1,2,3,4,5], n=2 → remove 4 → [1,2,3,5]
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
    }

    #[test]
    fn test_example_2() {
        // [1], n=1 → remove only node → []
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }

    #[test]
    fn test_example_3() {
        // [1,2], n=1 → remove last → [1]
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2]), 1),
            to_list(vec![1])
        );
    }

    #[test]
    fn test_remove_head() {
        // [1,2,3], n=3 → remove head → [2,3]
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3]), 3),
            to_list(vec![2, 3])
        );
    }

    #[test]
    fn test_remove_middle() {
        // [1,2,3,4,5], n=3 → remove 3 → [1,2,4,5]
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![1, 2, 4, 5])
        );
    }
}
