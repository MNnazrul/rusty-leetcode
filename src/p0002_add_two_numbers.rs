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

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut main_list = Box::new(ListNode::new(0));
    let mut carry = 0;
    let mut temp_list = &mut main_list;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        } else {
            l1 = None;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        } else {
            l2 = None;
        }

        carry = sum / 10;

        temp_list.next = Some(Box::new(ListNode::new(sum % 10)));
        temp_list = temp_list.next.as_mut().unwrap();
    }

    main_list.next
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
        // 342 + 465 = 807
        assert_eq!(
            add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        );
    }

    #[test]
    fn test_example_3() {
        // 9999999 + 9999 = 10009998
        assert_eq!(
            add_two_numbers(
                to_list(vec![9, 9, 9, 9, 9, 9, 9]),
                to_list(vec![9, 9, 9, 9])
            ),
            to_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
