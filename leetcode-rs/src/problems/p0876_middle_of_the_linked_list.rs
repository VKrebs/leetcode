pub struct Solution;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        // Advance while fast can take two steps
        while let Some(second) = fast.as_ref().and_then(|node| node.next.as_ref()) {
            fast = &second.next;
            slow = &slow.as_ref().expect("slow is always behind fast").next;
        }

        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;

        for val in v.into_iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }

        head
    }

    #[test]
    fn exemple_1() {
        let result = Solution::middle_node(to_list(vec![1, 2, 3, 4, 5]));
        assert_eq!(result, to_list(vec![3, 4, 5]));
    }

    #[test]
    fn exemple_2() {
        let result = Solution::middle_node(to_list(vec![1, 2, 3, 4, 5, 6]));
        assert_eq!(result, to_list(vec![4, 5, 6]));
    }

    #[test]
    fn single_node_is_its_own_middle() {
        let result = Solution::middle_node(to_list(vec![1]));
        assert_eq!(result, to_list(vec![1]));
    }

    #[test]
    fn two_nodes_returns_second() {
        let result = Solution::middle_node(to_list(vec![1, 2]));
        assert_eq!(result, to_list(vec![2]));
    }
}
